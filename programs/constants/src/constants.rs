use anchor_lang::solana_program::pubkey::Pubkey;

pub const MASTER_SEED: &str = "master";
pub const AUTOMATIC_DRAW_SEED: &str = "automatic_draw";
pub const ENTRY_SEED: &str = "entry";

pub const EXCLUDED_ADDRESSES: [Pubkey; 20] = [
    solana_program::pubkey!("GPMjfV4LTE3tnAeVHAbGS4krJkqaDw4ydFj15e7kjJTU"),
    solana_program::pubkey!("8PEc2ivbUPdNLYjCnGDzDZufaoZiK5vmaxfThDe3nnm7"),
    solana_program::pubkey!("3H5PtuLf6oj4syACUjtX8Q86LhWxZ4hQbC5nDeBGDQbj"),
    solana_program::pubkey!("6JukvoP9CezpKNJQt5LE4Rdisy2SC2PueW4vKmmUmv1f"),
    solana_program::pubkey!("8ezycB1desDXkbQN5mM8VFZ7mChEUmo3cAXZMDUA8tsD"),
    solana_program::pubkey!("D35tQ6uefH3DmcCvhMRt8V3qRTjy6xMjWL6GmGNa8DDn"),
    solana_program::pubkey!("75EyTNzER7KGCr3h58cGsNDjhZtfZsr42YXJU9qqcjkg"),
    solana_program::pubkey!("5RBYtsTivBW7cz51FzUvQ1Q4qWg7CkQVjy2gZ1FtM293"),
    solana_program::pubkey!("5T1XGBg9dp1uFykmZTmobH5oDVzAWGLGdZAARqpKPT59"),
    solana_program::pubkey!("FdgPzGvCknpJyhokF3g3bVyGaWRqCcCB7wdzB8xVpGQ6"),
    solana_program::pubkey!("HxJpBBXYE5aos4BQY27r2MjFzicxAjy8RhJ9U4vMvm6R"),
    solana_program::pubkey!("CfVBbVEBq8kp94dtKxyw8keCUFg4W1uSJs9hyA3y757k"),
    solana_program::pubkey!("2bFBUnR8hwqYtsbUUD6LNcJZCfx8KAEhEayo6JqNT3DH"),
    solana_program::pubkey!("149SNMYcWGt9hztcWGsioW9RFp7yNRRw3cRwyVrNs6Cv"),
    solana_program::pubkey!("C7V4A27RxuzuS7Qv3F6qPCdAW46voWCoUHhG4XQjzuEk"),
    solana_program::pubkey!("4ep47ejZ5HhwJWi7PoYQxUWRN4srDZLZ9n8BP4oYt8JY"),
    solana_program::pubkey!("2C6Rr1qJ7CGbDzreFsp1YaAfF52Jga55HijQNLDkbzae"),
    solana_program::pubkey!("DSNXQ1Nt4mNwPx4RWUEKNwFoneZn9oSokcgeN9RA2oYH"),
    solana_program::pubkey!("HnQ4TCJhttL33TtqSQKDwMi6qG9SuX184VBXx3agn5ur"),
    solana_program::pubkey!("CNa8eCs3HCFvGYjKsqnodGftunY6gZQtM4HcqbXGCmu6"),
];
