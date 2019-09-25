# regular-dicers-backend

![openalcoholics logo](.github/header_photo.png)

This backend keeps track of a group of users (users in a telegram chat) who attend an event (usually once a week)
at which they drink cocktails and dice for the price.

It's gonna be coupled with a [telegram bot](https://github.com/openalcoholics/regular_dicers_bot) as a frontend.

## Overview

### User

#### Relations

- User belongs to chat
- User attends event
- User has messages

### Chat

#### Relations

- Chat has users
- Chat has events

### ChatUser

This is a join table for `Chat` and `User` with additional information about the user and his role in the chat.

### Message

#### Relations

- (optionally) sent by a User
- Message belongs to a Chat

### Event

#### Relations

- Event belongs to a chat
- Event has users

### EventUser

This is a join table for `Event` and `User` with additional information about the user and his role in the event.
