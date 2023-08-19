## Commands

- [ ] Furry Image Stickers
- [ ] Fursona Commands
    - [ ] Tie a fursona to a user
    - [ ] Upload images to a fursona
    - [ ] Post images of a fursona
    - [ ] Let other users manage a fursona
- [ ] Database
    - [ ] Record the amount of commands run

## Data models

- Luro Settings
    - [ ] `Hashmap<str, usize` Record the amount of commands run
- Guild Settings
    - [ ] Logging
        - [ ] Control what events are logged, and to what channel
        - [ ] Choose a log channel
- User Settings
    - [ ] Luro Overrides
    - [ ] Fursonas

## Notes
- 'Control what events are logged, and to what channel'
    - Model dropdown where a user can select what should be logged, per channel
    - Should be stored in a hashmap(?)

## Character / Fursona Command
- Character Profile - Ability to pull up a character profile
- Character Modify - Modify a profile that the user owns
    - Modify Image - Add or remove images tied to a character
    - Modify Settings - Sets some settings for a character

### Character - Profile
`/character profile name: Nurah nsfw: true, user: @nurah`

**Arguments:** [`name - The profile to fetch`, `nsfw - fetch NSFW version`, `<user - Fetch the profile belonging to another user>`]

Defaults to the user's own profile, otherwise fetches the specified profile name belonging to another user

Error handling:
- Returns a list of profiles configured if the name does not match

### Character - Modify
Simply a command group with more specific sub commands

#### Character - Image
`/character modify name: Nurah nsfw: true, user: @nurah`

**Arguments:** [`name - The profile to fetch`, `nsfw - fetch NSFW version`, `user - Fetch the profile belonging to another user`]
Simply a command group with more specific sub commands