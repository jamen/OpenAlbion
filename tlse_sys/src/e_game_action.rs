#[derive(Debug)]
#[repr(C)]
pub enum EGameAction {
	GAME_ACTION_NULL = 0,
	GAME_ACTION_LOCK_TARGET = 1,
	GAME_ACTION_OPEN_INVENTORY = 2,
	GAME_ACTION_OPEN_IN_GAME_MENU = 3,
	GAME_ACTION_TOGGLE_MINI_MAP = 4,
	GAME_ACTION_PAUSE = 5,
	GAME_ACTION_INTERACT = 6,
	GAME_ACTION_BLOCK = 7,
	GAME_ACTION_SPECIAL_ATTACK = 8,
	GAME_ACTION_ATTACK = 9,
	GAME_ACTION_FIRE_RANGED_WEAPON = 10,
	GAME_ACTION_UNARMED_ATTACK = 11,
	GAME_ACTION_ARMED_MELEE_ATTACK = 12,
	GAME_ACTION_UNSHEATHE_MELEE_WEAPON = 13,
	GAME_ACTION_UNSHEATHE_RANGED_WEAPON = 14,
	GAME_ACTION_SHEATHE_MELEE_WEAPON = 15,
	GAME_ACTION_SHEATHE_RANGED_WEAPON = 16,
	GAME_ACTION_TOGGLE_SILENT_MOVE = 17,
	GAME_ACTION_TOGGLE_CINEMATIC_AND_USER_CAMERA = 18,
	GAME_ACTION_TOGGLE_FIRST_PERSON_VIEW = 19,
	GAME_ACTION_SKIP_PAST_TEXT = 20,
	GAME_ACTION_SKIP_CUT_SCENE = 21,
	GAME_ACTION_ANSWER_QUESTION_YES = 22,
	GAME_ACTION_ANSWER_QUESTION_NO = 23,
	GAME_ACTION_ANSWER_QUESTION_THIRD = 24,
	GAME_ACTION_INVENTORY_SELECT = 25,
	GAME_ACTION_ATTRACT_EXPERIENCE_ORBS = 26,
	GAME_ACTION_ROTATE_GUI_SCREENS_LEFT = 27,
	GAME_ACTION_ROTATE_GUI_SCREENS_RIGHT = 28,
	GAME_ACTION_JUMP = 29,
	GAME_ACTION_SPRINT = 30,
	GAME_ACTION_RUN = 31,
	GAME_ACTION_SNEAK = 32,
	GAME_ACTION_INVENTORY_A = 33,
	GAME_ACTION_INVENTORY_B = 34,
	GAME_ACTION_INVENTORY_X = 35,
	GAME_ACTION_INVENTORY_Y = 36,
	GAME_ACTION_INVENTORY_UP = 37,
	GAME_ACTION_INVENTORY_DOWN = 38,
	GAME_ACTION_INVENTORY_LEFT = 39,
	GAME_ACTION_INVENTORY_RIGHT = 40,
	GAME_ACTION_INVENTORY_WHITE = 41,
	GAME_ACTION_TAVERN_GAMES_INSTRUCTIONS = 42,
	GAME_ACTION_FISHING_REEL_IN = 43,
	GAME_ACTION_FISHING_CANCEL = 44,
	GAME_ACTION_TOGGLE_FIRST_PERSON_TARGETING = 45,
	GAME_ACTION_FIRST_PERSON_TARGET_LOCK = 46,
	GAME_ACTION_FIRST_PERSON_ZOOM_IN = 47,
	GAME_ACTION_GENERAL_LEAVE_PLAYER_MODE = 48,
	GAME_ACTION_DEBUG_JUMP_1 = 49,
	GAME_ACTION_DEBUG_JUMP_2 = 50,
	GAME_ACTION_DEBUG_CAMERA = 51,
	GAME_ACTION_DEBUG_SHIFT = 52,
	GAME_ACTION_TAKE_PHOTO_FOR_PHOTOJOURNAL = 53,
	GAME_ACTION_ASSIGNABLE_SPECIAL_MOVE = 54,
	GAME_ACTION_QUICK_ACCESS_ITEM = 55,
	GAME_ACTION_CONTEXT_SENSITIVE_ITEM = 56,
	GAME_ACTION_CYCLE_THROUGH_SPELLS = 57,
	GAME_ACTION_COIN_GOLF_CANCEL_AIM = 58,
	GAME_ACTION_CONFIRM_RESET_TO_FRONT_END = 59,
	GAME_ACTION_MOVEMENT = 60,
	GAME_ACTION_CAMERA_ROTATE = 61,
	GAME_ACTION_CAMERA_ROTATE_LEFT = 62,
	GAME_ACTION_CAMERA_ROTATE_RIGHT = 63,
	GAME_ACTION_CAMERA_ZOOM_IN = 64,
	GAME_ACTION_CAMERA_ZOOM_OUT = 65,
	GAME_ACTION_ORACLE_MINIGAME_UP = 66,
	GAME_ACTION_ORACLE_MINIGAME_DOWN = 67,
	GAME_ACTION_ORACLE_MINIGAME_LEFT = 68,
	GAME_ACTION_ORACLE_MINIGAME_RIGHT = 69,
	GAME_ACTION_ORACLE_MINIGAME_QUIT = 70,
	GAME_ACTION_MOVE_MOUSE_ON_GUI = 71,
	GAME_ACTION_TOGGLE_LIVE_GUI = 72,
	GAME_ACTION_OPEN_EXPRESSION_MENU = 73,
	GAME_ACTION_TOGGLE_DEACTIVATE_LOCK_TARGET = 74,
	GAME_ACTION_FIRST_PERSON_LOOKAROUND = 75,
	GAME_ACTION_INVENTORY_UNSELECT = 76,
	GAME_ACTION_CAMERA_MOVE_DOUBLE_AXIS = 77,
	GAME_ACTION_CHARGE_GUILD_SEAL = 78,
	GAME_ACTION_TAVERN_GAME_MOVEMENT = 79,
	GAME_ACTION_TAVERN_GAME_CAMERA = 80,
	GAME_ACTION_TAVERN_GAME_ACTION_BUTTON = 81,
	GAME_ACTION_TAVERN_GAME_ALTERNATE_BUTTON = 82,
	GAME_ACTION_TAVERN_GAME_QUIT = 83,
	GAME_ACTION_PROJECTILE_TARGETING_ANALOGUE_ZOOM = 84,
	GAME_ACTION_TOGGLE_PASSIVE_AGGRESSIVE_MODE = 85,
	GAME_ACTION_ACTIVATE_SPELL_MODE = 86,
	GAME_ACTION_EXPRESSION_SHIFT = 87,
	GAME_ACTION_SCROLL_DESCRIPTION_UP = 88,
	GAME_ACTION_SCROLL_DESCRIPTION_DOWN = 89,
	GAME_ACTION_OPEN_WEAPONS_MENU = 90,
	GAME_ACTION_OPEN_CLOTHING_MENU = 91,
	GAME_ACTION_OPEN_ITEMS_MENU = 92,
	GAME_ACTION_OPEN_CURRENT_QUESTS_MENU = 93,
	GAME_ACTION_CYCLE_THROUGH_SPELLS_KEYBOARD = 94,
	GAME_ACTION_TOGGLE_KILL_EVERYTHING_MODE = 95,
	GAME_ACTION_OPEN_MAGIC_MENU = 96,
	GAME_ACTION_OPEN_EXPRESSIONS_MENU = 97,
	GAME_ACTION_OPEN_PERSONALITY_MENU = 98,
	GAME_ACTION_OPEN_LOGBOOK_MENU = 99,
	GAME_ACTION_OPEN_MAP_MENU = 100,
	GAME_ACTION_SCROLL_MENU = 101,
	GAME_ACTION_ZOOM_MAP_OUT = 102,
	GAME_ACTION_ZOOM_MAP_IN = 103,
	GAME_ACTION_SCROLL_MAP_LEFT = 104,
	GAME_ACTION_SCROLL_MAP_RIGHT = 105,
	GAME_ACTION_SCROLL_MAP_DOWN = 106,
	GAME_ACTION_SCROLL_MAP_UP = 107,
	GAME_ACTION_OPTIONS_SLIDER_LEFT = 108,
	GAME_ACTION_OPTIONS_SLIDER_RIGHT = 109,
	GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_IN = 110,
	GAME_ACTION_DIGITAL_ANALOGUE_ZOOM_OUT = 111,
	GAME_ACTION_TOGGLE_VIEW_HERO_MODE = 112,
	GAME_ACTION_CENTRE_CAMERA = 113,
	GAME_ACTION_BETTING = 114,
	GAME_ACTION_COUNT = 115,
}