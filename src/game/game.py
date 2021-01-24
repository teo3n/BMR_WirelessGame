from __future__ import annotations

from time import sleep
from datetime import datetime
import math
from typing import List

BOARD_WIDTH = 16

board = ["X" for x in range(BOARD_WIDTH * BOARD_WIDTH)]


def quad(a, b, c):
    determinant = b * b - 4 * a * c
    # Avoid math domain error. Since we only care about collisions that happen between t=[0, 1)
    # it is ok to return -1 here
    if determinant < 0:
        return -1
    else:
        # Similarly we only care about the first touching so we don't have to care about the
        # positive root
        return (-b - math.sqrt(determinant)) / (2 * a)


def collision(start1, v1, start2, v2, radius=1):
    # solves the following equation (x'(0) - x'(1))^2 + (y'(0) - y'(1))^2 = radius ^ 2 regards to time
    # Basically when this equation equals to zero the two objects touch.
    a = (v1[0] - v2[0]) ** 2 + (v1[1] - v2[1]) ** 2
    b = -2 * (v1[0] - v2[0]) * (start2[0] - start1[0]) - 2 * (v1[1] - v2[1]) * (start2[1] - start1[1])
    c = (start2[0] - start1[0]) ** 2 + (start2[1] - start1[1]) ** 2 - radius ** 2
    if a == 0 and b:
        return -c / b
    if a == 0 and b == 0:
        return -1
    return quad(a, b, c)


class MovingObject:
    deceleration = -0.1
    BOARD_INSIDE = BOARD_WIDTH - 3

    def __init__(self, starting_velocity=(0, 0), symbol="*"):
        self.velocity = list(starting_velocity)
        self.location = list((self.BOARD_INSIDE / 2, self.BOARD_INSIDE / 2))
        total = sum(abs(x) for x in self.velocity)
        if total:
            self.ratios = [abs(x) / total for x in self.velocity]
        else:
            # Just in case we have a stationary object from the start
            # Shouldn't happen but just in case.
            self.ratios = [0.5, 0.5]
        self.symbol = symbol

    def tick(self, time: float, final: bool, collision_velocity=None):
        self.location = [
            self.location[0] + self.velocity[0] * time,
            self.location[1] + self.velocity[1] * time
        ]

        if collision_velocity:
            self.velocity = collision_velocity

        if final:
            if not self.moving:
                return
            if self.velocity[0] < 0:
                temp = min(0, self.velocity[0] - self.deceleration * self.ratios[0])
                assert abs(temp) <= abs(self.velocity[0])
                self.velocity[0] = temp
            else:
                temp = max(0, self.velocity[0] + self.deceleration * self.ratios[0])
                assert abs(temp) <= abs(self.velocity[0])
                self.velocity[0] = temp

            if self.velocity[1] < 0:
                temp = min(0, self.velocity[1] - self.deceleration * self.ratios[1])
                assert abs(temp) <= abs(self.velocity[1])
                self.velocity[1] = temp
            else:
                temp = max(0, self.velocity[1] + self.deceleration * self.ratios[1])
                assert abs(temp) <= abs(self.velocity[1])
                self.velocity[1] = temp

    def get_collisions(self, others: List[MovingObject], max_duration=1.0):
        new_loc = [
            self.location[0] + self.velocity[0],
            self.location[1] + self.velocity[1]
        ]
        collisions = []

        if new_loc[0] < 0 and abs(self.location[0] / self.velocity[0]) < max_duration:
            collisions.append(("LEFT_WALL", abs(self.location[0] / self.velocity[0]), self))

        if new_loc[0] > self.BOARD_INSIDE and abs(
                (self.BOARD_INSIDE - self.location[0]) / self.velocity[0]) < max_duration:
            collisions.append(("RIGHT_WALL", abs((self.BOARD_INSIDE - self.location[0]) / self.velocity[0]), self))

        if new_loc[1] < 0 and abs(self.location[1] / self.velocity[1]) < max_duration:
            collisions.append(("TOP_WALL", abs(self.location[1] / self.velocity[1]), self))

        if new_loc[1] > self.BOARD_INSIDE and abs(
                (self.BOARD_INSIDE - self.location[1]) / self.velocity[1]) < max_duration:
            collisions.append(("BOTTOM_WALL", abs((self.BOARD_INSIDE - self.location[1]) / self.velocity[1]), self))

        for other in others:
            if not self.moving and not other.moving:
                continue
            collision_time = collision(self.location, self.velocity, other.location, other.velocity)
            if 0 < collision_time < max_duration:
                collisions.append((other, collision_time, self))

        return collisions

    @property
    def position(self):
        return [int(round(self.location[0])) + 1, int(round(self.location[1])) + 1]

    @property
    def moving(self):
        return bool(self.velocity[0] or self.velocity[1])


def print_board(board):
    for y in range(BOARD_WIDTH):
        for x in range(BOARD_WIDTH):
            print(board[x + y * BOARD_WIDTH], end=" ")
        print()


def game():
    clear_board(board)

    tba_objects = []
    # the file should have one line per object space separated with three fields:
    # "how many game ticks to wait since last object before starting" "starting angle" "symbol"
    with open("tba_objects.txt") as f:
        for line in f:
            temp = line.strip().split()
            temp[0] = int(temp[0])
            temp[1] = int(temp[1])
            tba_objects.append(temp)

    objects = []
    while tba_objects[0][0] == 0:
        _, angle, symbol = tba_objects.pop(0)
        objects.append(MovingObject((math.cos(angle) * 5, -math.sin(angle) * 5), symbol))

    while True:
        start = datetime.now()
        print_board(board)

        tick_so_far = 0
        while tick_so_far < 1:
            col_vels = [None for _ in range(len(objects))]
            all_collision = []
            for i in range(len(objects)):
                ob = objects[i]
                collisions = ob.get_collisions(objects[i + 1:], 1 - tick_so_far)
                all_collision.extend(collisions)
            dur = 1 - tick_so_far
            if all_collision:
                # Allows processing simultaneously happening collisions and ensures that there are maximum of
                # 100 events in one game tick.
                # One problem with this method is that if same object has two collisions rapidly one of them can
                # be ignored, this will make the simulation not accurate but there should be no real problems
                # since even if the object ends up outside of the board and inside the wall it will be constantly
                # colliding with the wall until it is outside of the wall
                first_collision_time = math.ceil(min(all_collision, key=lambda x: x[1])[1] * 100) / 100
                used_collisions = []
                dur = first_collision_time
                for coll in all_collision:
                    if coll[1] <= first_collision_time:
                        used_collisions.append(coll)

                for coll in used_collisions:
                    other_object, _, collider = coll
                    idx = objects.index(collider)
                    if other_object == "LEFT_WALL" or other_object == "RIGHT_WALL":
                        col_vels[idx] = [-collider.velocity[0], collider.velocity[1]]
                    elif other_object == "TOP_WALL" or other_object == "BOTTOM_WALL":
                        col_vels[idx] = [collider.velocity[0], -collider.velocity[1]]
                    else:
                        other_idx = objects.index(other_object)
                        if other_object.moving and collider.moving:
                            # I'm not sure if this is 100% accurate way for two round objects with same mass
                            # but seems close enough
                            temp = collider.ratios
                            collider.ratios = other_object.ratios
                            other_object.ratios = temp
                            total_velocity = (math.sqrt(collider.velocity[0] ** 2 + collider.velocity[1] ** 2) +
                                              math.sqrt(
                                                  other_object.velocity[0] ** 2 + other_object.velocity[1] ** 2)) / 2
                            col_vels[other_idx] = [
                                total_velocity * other_object.ratios[0] * (-1 if collider.velocity[0] < 0 else 1),
                                total_velocity * other_object.ratios[1] * (-1 if collider.velocity[1] < 0 else 1)]
                            col_vels[idx] = [
                                total_velocity * collider.ratios[0] * (-1 if other_object.velocity[0] < 0 else 1),
                                total_velocity * collider.ratios[1] * (-1 if other_object.velocity[1] < 0 else 1)]
                        # Stationary objects remain stationary because I just want them to
                        else:
                            if collider.moving:
                                col_vels[idx] = [-collider.velocity[0], -collider.velocity[1]]
                            else:
                                col_vels[other_idx] = [-other_object.velocity[0], -other_object.velocity[1]]

            for ob, col_vel in zip(objects, col_vels):
                ob.tick(dur, not (tick_so_far + dur < 1), col_vel)

            if all_collision:
                for coll in used_collisions:
                    other_object, _, collider = coll
                    if other_object == "LEFT_WALL":
                        board[0 + int(collider.location[1] + 1.5) * BOARD_WIDTH] = collider.symbol
                    elif other_object == "RIGHT_WALL":
                        board[BOARD_WIDTH - 1 + int(collider.location[1] + 1.5) * BOARD_WIDTH] = collider.symbol
                    elif other_object == "TOP_WALL":
                        board[0 + int(collider.location[0] + 1.5)] = collider.symbol
                    elif other_object == "BOTTOM_WALL":
                        board[(BOARD_WIDTH - 1) * BOARD_WIDTH + int(collider.location[0] + 1.5)] = collider.symbol
            tick_so_far += dur

        clear_board(board)

        if tba_objects and tba_objects[0][0] == 0:
            _, angle, symbol = tba_objects.pop(0)
            objects.append(MovingObject((math.cos(angle) * 5, -math.sin(angle) * 5), symbol))

        for ob in objects:
            board[ob.position[0] + ob.position[1] * BOARD_WIDTH] = ob.symbol

        if not any(x.moving for x in objects) and not tba_objects:
            break

        if tba_objects:
            tba_objects[0][0] -= 1
        stop = datetime.now()

        sleep(0.5 - (stop - start).microseconds / 1000000)


def clear_board(board):
    for y in range(BOARD_WIDTH):
        for x in range(BOARD_WIDTH):
            if y != 0 and y != BOARD_WIDTH - 1 and x != 0 and x != BOARD_WIDTH - 1:
                board[x + y * BOARD_WIDTH] = "."


if __name__ == '__main__':
    game()
