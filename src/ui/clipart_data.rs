pub struct ClipartEntry {
    pub id: &'static str,
    pub label: &'static str,
    pub category: &'static str,
    pub png_bytes: &'static [u8],
}

pub const CAT_ALL: &str = "All";
pub const CAT_SHIPPING: &str = "Shipping";
pub const CAT_CRAFT: &str = "Craft";
pub const CAT_ANIMALS: &str = "Animals";

pub static CLIPART: &[ClipartEntry] = &[
    ClipartEntry {
        id: "local_shipping",
        label: "Shipping",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/local_shipping.png"),
    },
    ClipartEntry {
        id: "package",
        label: "Package",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/package.png"),
    },
    ClipartEntry {
        id: "location_on",
        label: "Location",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/location_on.png"),
    },
    ClipartEntry {
        id: "sell",
        label: "Price Tag",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/sell.png"),
    },
    ClipartEntry {
        id: "barcode",
        label: "Barcode",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/barcode.png"),
    },
    ClipartEntry {
        id: "scissors",
        label: "Scissors",
        category: CAT_SHIPPING,
        png_bytes: include_bytes!("../../assets/clipart/scissors.png"),
    },
    ClipartEntry {
        id: "star",
        label: "Star",
        category: CAT_CRAFT,
        png_bytes: include_bytes!("../../assets/clipart/star.png"),
    },
    ClipartEntry {
        id: "favorite",
        label: "Heart",
        category: CAT_CRAFT,
        png_bytes: include_bytes!("../../assets/clipart/favorite.png"),
    },
    ClipartEntry {
        id: "check_circle",
        label: "Check",
        category: CAT_CRAFT,
        png_bytes: include_bytes!("../../assets/clipart/check_circle.png"),
    },
    ClipartEntry {
        id: "arrow_forward",
        label: "Arrow",
        category: CAT_CRAFT,
        png_bytes: include_bytes!("../../assets/clipart/arrow_forward.png"),
    },
    ClipartEntry {
        id: "grade",
        label: "Award",
        category: CAT_CRAFT,
        png_bytes: include_bytes!("../../assets/clipart/grade.png"),
    },
    ClipartEntry {
        id: "pets",
        label: "Paw",
        category: CAT_ANIMALS,
        png_bytes: include_bytes!("../../assets/clipart/pets.png"),
    },
    ClipartEntry {
        id: "cruelty_free",
        label: "Bunny",
        category: CAT_ANIMALS,
        png_bytes: include_bytes!("../../assets/clipart/cruelty_free.png"),
    },
    ClipartEntry {
        id: "cat",
        label: "Cat",
        category: CAT_ANIMALS,
        png_bytes: include_bytes!("../../assets/clipart/cat.png"),
    },
    ClipartEntry {
        id: "set_meal",
        label: "Fish",
        category: CAT_ANIMALS,
        png_bytes: include_bytes!("../../assets/clipart/set_meal.png"),
    },
];

pub fn find(id: &str) -> Option<&'static ClipartEntry> {
    CLIPART.iter().find(|e| e.id == id)
}

pub fn categories() -> &'static [&'static str] {
    &[CAT_ALL, CAT_SHIPPING, CAT_CRAFT, CAT_ANIMALS]
}
