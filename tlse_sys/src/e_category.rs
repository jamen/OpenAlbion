#[derive(Debug)]
#[repr(C)]
pub enum ECategory {
	CATEGORY_QUEST = 0,
	CATEGORY_STORY = 1,
	CATEGORY_TUTORIAL = 2,
	CATEGORY_BASICS = 3,
	CATEGORY_OBJECTS = 4,
	CATEGORY_TOWNS = 5,
	CATEGORY_HERO = 6,
	CATEGORY_COMBAT = 7,
}