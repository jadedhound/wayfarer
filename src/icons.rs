// -----------------------------------
// HEROICONS
// -----------------------------------

pub const MINUS: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width="3">
      <path d="M19.5 12h-15" />
    </svg>
"#;
pub const PLUS: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width="3">
      <path d="M12 4.5v15m7.5-7.5h-15" />
    </svg>
"#;
pub const TRASH: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width=0>
      <path fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 013.878.512.75.75 0 11-.256 1.478l-.209-.035-1.005 13.07a3 3 0 01-2.991 2.77H8.084a3 3 0 01-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 01-.256-1.478A48.567 48.567 0 017.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 013.369 0c1.603.051 2.815 1.387 2.815 2.951zm-6.136-1.452a51.196 51.196 0 013.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 00-6 0v-.113c0-.794.609-1.428 1.364-1.452zm-.355 5.945a.75.75 0 10-1.5.058l.347 9a.75.75 0 101.499-.058l-.346-9zm5.48.058a.75.75 0 10-1.498-.058l-.347 9a.75.75 0 001.5.058l.345-9z" clip-rule="evenodd" />
    </svg>
"#;
pub const ELLIPSES: &str = r#"
    <svg fill="none" viewBox="0 0 24 24" stroke-width="1.5">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z" />
    </svg>
"#;
pub const HOME: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width=0>
      <path d="M11.47 3.84a.75.75 0 011.06 0l8.69 8.69a.75.75 0 101.06-1.06l-8.689-8.69a2.25 2.25 0 00-3.182 0l-8.69 8.69a.75.75 0 001.061 1.06l8.69-8.69z" />
      <path d="M12 5.432l8.159 8.159c.03.03.06.058.091.086v6.198c0 1.035-.84 1.875-1.875 1.875H15a.75.75 0 01-.75-.75v-4.5a.75.75 0 00-.75-.75h-3a.75.75 0 00-.75.75V21a.75.75 0 01-.75.75H5.625a1.875 1.875 0 01-1.875-1.875v-6.198a2.29 2.29 0 00.091-.086L12 5.43z" />
    </svg>
"#;

// -----------------------------------
// SIMPLE
// -----------------------------------

pub const STACK: &str = r#"
    <svg viewBox="0 0 512 512">
      <path d="M256 30 10 138l-6 7-4 10 2 11 6 7 240 129 8 1 8-1 240-129 6-8 2-10-3-9-7-7zM8 266s242 130 247 130c3 0 247-128 249-130 2-1 8-6 8-16 0-8-4-17-12-20-9-4-16-2-16-2L256 349 29 228s0-2-15 1c-8 1-14 14-14 22 0 10 8 15 8 15zm248 216c3 0 247-129 250-132 2-3 6-10 6-15 0-7-3-12-8-16-7-7-14-6-17-6L256 435A3410 3410 0 0 0 24 313c-3 0-2-1-11 2-5 1-13 11-13 19 0 9 5 15 7 17l249 131z"/>
    </svg>
"#;
pub const BOLT: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width="1.5" fill="none">
        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
    </svg>
"#;
pub const CHECKMARK: &str = r#"
    <svg viewBox="0 0 512 512">
        <path d="M373 45c-27 25-93 113-149 198L124 395c-32-71-54-90-66-92-13 0-32 11-45 23-17 20-17 32 8 101 24 72 30 79 71 81 40 4 49-2 115-103 38-59 117-170 177-249C442 77 491 11 491 6c0-4-15-6-35-6-16 0-55 19-83 45z"/>
    </svg>
"#;
pub const CROSS: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width=0>
        <path d="m0 6 7 6-7 7 6 5 6-6 7 6 5-5-7-7 7-6-5-6-7 7-6-7z"/>
    </svg>
"#;
pub const CIRCLE: &str = r#"
    <svg viewBox="0 0 24 24" stroke-width=0>
      <path d="M12 24C5 24 0 19 0 12S5 0 12 0s12 5 12 12-5 12-12 12z"/>
    </svg>
"#;
pub const RIGHT_CHEV: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path d="m0 73 184 183L0 440l70 70 252-254L70 2zM252 4l-71 69 184 185-184 184 71 68 260-252z"/>
    </svg>
"#;
pub const STAR: &str = r#"
    <svg viewBox="0 0 128 128" stroke-width=15>
        <path fill-rule="evenodd" d="M70 12c-1-2-4-4-6-4-3 0-6 2-6 4L44 41l-30 5c-3 0-5 2-6 4-1 3 0 5 2 7l22 23-5 32c-1 2 1 5 2 7 2 2 6 2 7 1l28-16 27 16c2 1 5 1 7-1s4-5 3-7l-6-32 23-23c1-2 2-4 1-7-1-2-2-4-5-4l-31-5z"/>
    </svg>
"#;

// -----------------------------------
// COMPLEX
// -----------------------------------

pub const CLOCK: &str = r#"
    <svg viewBox="0 0 512 512">
    	<path fill-rule="evenodd" d="m256 512c-141.6 0-256-114.4-256-256 0-141.6 114.4-256 256-256 141.6 0 256 114.4 256 256 0 141.6-114.4 256-256 256zm221-256c0-122.2-98.8-221-221-221-122.2 0-221 98.8-221 221 0 122.2 98.8 221 221 221 122.2 0 221-98.8 221-221zm-239 4l2-148c0 0 5-10 16-10 10 0 16 10 16 10v134.8l85 70.2c0 0 4 10-3 18-7.7 8.8-20 4-20 4l-95.2-78.3-0.8 0.3v-1z"/>
    </svg>
"#;
pub const INFO: &str = r#"
    <svg viewBox="0 0 96 96" stroke-width="14.7" stroke-linecap="round">
      <path d="M56 84h-1l-4 1-4-2-2-3v-4l7-27-1-4-2-4a8 8 0 0 0-8 0h-1m8-29h0v0h0z" />
    </svg>
"#;
pub const QUILL: &str = r#"
    <svg viewBox="0 0 489 489">
      <path fill-rule="evenodd" d="M54 489c-11 0-20-11-20-22l1-7 75-147c22-46 62-107 112-163C309 51 429-16 450 5c5 5 6 14 3 26-7 8-38 44-52 69-20 32-31 69-31 69l2-2-2 3c-3 0-59 2-62 9s21 43 22 44l-6 8a499 499 0 0 1-200 160l-51 88h-1c-4 5-11 9-18 9zm307-307-23 31z"/>
    </svg>
"#;
pub const REFRESH: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width="0">
        <path d="M2 312v164c0 14 23 19 29 12l48-46s24 28 95 55c65 25 168 15 235-36 78-60 95-152 94-155-1-2-78-7-81-5-1 2-27 100-124 119-109 21-158-39-158-39 41-41 38-37 50-52 7-9-3-27-16-28H13c-11 0-11 1-11 11zm510-112-1-164c0-14-23-19-29-12-8 10-47 46-48 46 0 0-24-28-95-55-65-25-168-15-235 36-78 60-95 152-93 155 0 2 77 7 80 5 1-2 27-100 124-119 109-21 158 39 158 39-40 41-38 37-50 52-7 9 3 27 16 28h162c10 0 10-1 11-11z"/>
    </svg>
"#;
// -----------------------------------
// CLASS
// -----------------------------------

pub const WARRIOR: &str = r#"
    <svg viewBox="0 0 666 666" stroke-width=0>
      <path fill-rule="evenodd" d="m36 20 8 2 4 3 2 3-7 6a123 123 0 0 0-28 28l-6 7-4-3-3-7c-1-3-2-7-1-10l2-9 7-9 9-8 8-3h9zm601 0 9 2 9 7 8 9c2 5 3 7 3 13s-1 8-3 13l-5 5-6-6a295 295 0 0 0-36-36l5-4 8-3h8zM61 37l85 85 7-6a392 392 0 0 1 41-27l-3-50h9l95 96-2 2a205 205 0 0 0-80 32c0 1 108 109 106 111l-25 26-112-110-3 2-2 3 109 111-26 27c-1 0-109-107-111-106l-7 11a195 195 0 0 0-21 51l-3 15-1 3-96-94-1-5 2-4 46 3c2 0 4-2 7-8a374 374 0 0 1 27-40L18 80l1-3 6-10a127 127 0 0 1 27-25l9-5zm545 0 9 6 16 12a109 109 0 0 1 17 25l-84 85 6 9a236 236 0 0 1 21 31c4 6 5 8 7 8l47-3 1 6c-1 4-96 98-97 96a334 334 0 0 1-7-28 279 279 0 0 0-18-41l-7-10c-2-1-38 35-356 353l-83 22v-3l21-76c1-5 3-7 356-360l-3-2-19-10a278 278 0 0 0-49-19l-11-2-2-1 95-96h5c4 0 5 0 5 2l-4 47 7 6a678 678 0 0 1 26 17l15 10 86-84zM255 70l3-2a323 323 0 0 1 111-7l24 3 15 4 4 2-62 63h-33l-62-63zm83 160 16 1 11 4-32 31-31-31 5-3 13-2h18zM51 275l63 62v12a307 307 0 0 0 5 48 223 223 0 0 0 21 58v5s-53 52-55 51a156 156 0 0 1-15-28 314 314 0 0 1-27-82 363 363 0 0 1-3-71 330 330 0 0 1 11-55zm564 0 2 2 3 11a275 275 0 0 1 2 122 460 460 0 0 1-15 50 432 432 0 0 1-27 51c-1 1-53-52-53-53-1-1 0-4 2-8a257 257 0 0 0 22-75l1-38 63-62zm-165 46 2 1 3 12 1 24-2 17-1 7-2 2-32-31 31-32zm-234 1 30 30a641 641 0 0 1-32 30l-2-9a166 166 0 0 1 0-41l2-8 2-2zm191 43 160 160 21 75 1 6-1 2-83-22-159-160 60-61zm-74 73a935 935 0 0 1 30 34 98 98 0 0 1-30 3 150 150 0 0 1-31-4c-1-1 31-33 31-33zm107 106 54 54-6 4a345 345 0 0 1-65 30 308 308 0 0 1-122 13 388 388 0 0 1-72-18 312 312 0 0 1-55-29l53-53 13 5a291 291 0 0 0 41 15 282 282 0 0 0 58 6 185 185 0 0 0 71-14 226 226 0 0 0 30-13zm-86-215c0 3 0 3 3 2 3 0 133-130 132-132l-3-3c-2 0-132 130-132 133z"/>
    </svg>
"#;
pub const CLERIC: &str = r#"
    <svg viewBox="0 0 731 731" stroke-width=0>
      <path d="M583 298c-2 1-2 2 1 13l5 23c1 8 2 17 1 32a195 195 0 0 1-8 63c-1 3-1 3 2 4l3-1 6-6a161 161 0 0 1 62-44 228 228 0 0 1 50-13l16-1h8v-5a280 280 0 0 1-45-6 250 250 0 0 1-54-22 259 259 0 0 1-35-28l-8-9-1-1-3 1zM364 11a337 337 0 0 1-8 45 198 198 0 0 1-30 60 198 198 0 0 1-24 26l-3 5 1 3 13-3 23-4c8-2 17-2 32-2a210 210 0 0 1 52 6c12 3 13 3 14 1v-3l-4-4-13-12-16-20a237 237 0 0 1-22-46 271 271 0 0 1-13-63c-2 0-2 1-2 11zM81 100l155 186 50 1v-52A13108 13108 0 0 0 81 100zm396 114-30 21v51l49 1a35916 35916 0 0 0 154-187L477 214zm-128-32-14 1a735 735 0 0 0-22 5c-1 1 10 23 10 23l-1 110H210s-21-11-22-10l-2 7a237 237 0 0 0-1 80 188 188 0 0 0 4 22l23-11 110 1 1 110-10 23 7 3 20 2a346 346 0 0 0 80-4l1-3-10-19V410l111-1 22 11 2-3 2-15a285 285 0 0 0-3-91c-1-1-23 10-23 10l-111-2-1-109 11-22a220 220 0 0 0-72-6zM139 306a190 190 0 0 1-30 26 249 249 0 0 1-43 21 208 208 0 0 1-57 10l-7 2v2l7 1a176 176 0 0 1 35 4 224 224 0 0 1 45 16l22 12 24 21c11 11 14 13 15 12l2-2-3-12a244 244 0 0 1-6-50 263 263 0 0 1 4-47 241 241 0 0 1 4-22l-2-2c-1-1-5 3-10 8zm-37 339-2 5 184-155c2-2 2-51 2-51h-53L102 645zm345-201v50a212403 212403 0 0 0 187 156v-1l-5-7-122-187-7-11-53 1zM299 582c-1 2 0 3 12 15a185 185 0 0 1 30 40 211 211 0 0 1 21 59l2 21c1 13 1 13 3 13s2 0 2-4l1-9a303 303 0 0 1 9-50 216 216 0 0 1 21-44 218 218 0 0 1 25-29c7-7 9-9 9-11s-1-2-3-2l-10 2a252 252 0 0 1-73 6 352 352 0 0 1-45-8c-2-1-3-1-4 1z"/>
    </svg>
"#;
pub const MAGE: &str = r#"
    <svg viewBox="0 0 738 738" stroke-width=0>
      <path d="M251 54c6 25 26 35 26 35l35-29s-20-9-33-20c-10-9-21-39-21-39s-15 19-7 53zm-48 122-7 54 117 93 15-6 43-65-22-26-37 23-24-27 57-53 86 76-186 233 59 69 26 190 3 1 21-9 20-188-43-59 176-199-2-103L346 60zm445 234a292 292 0 0 1-44 208c1 0 47-32 75-96 25-58 22-147-20-207-40-57-118-98-119-98 0 0 91 76 108 193zM64 364c-26 58-21 79-20 118 1 47 31 95 69 137 33 39 99 68 99 68s-106 16-105 17l202 24-21-166s-46-15-68-37c-17-16-48-55-49-97-1-64 29-99 29-99 0-1-36 20-59 59-27 46-28 112-28 112s-11-35-11-71c0-28 4-53 11-79 11-46 47-104 47-104s-62 41-96 118zm439-36s27 54 28 98c0 59-32 94-32 94s4-47-2-75c-9-50-26-71-26-71s15 49-8 101c-16 39-65 72-66 72-2 1-21 179-19 180l198-23-105-17s42-27 80-74c34-42 53-82 58-115 7-48 2-97-19-139-10-20-52-82-52-82z"/>
    </svg>
"#;
pub const ROGUE: &str = r#"
    <svg viewBox="0 0 698 698" stroke-width=0>
      <path fill-rule="evenodd" d="m349 1 5 3a411 411 0 0 1 28 20 128 128 0 0 1 21 26l6 12v7l-60 57s-61-58-61-60c1-2 2-7 5-11l9-15a142 142 0 0 1 28-28l13-8zm-34 118 34 31 34-31 8 87q0 7 2 8l5 1 15 2a275 275 0 0 1 44 12 190 190 0 0 1 33 18 127 127 0 0 1 24 25l7 13a115 115 0 0 1 8 32v22a220 220 0 0 1-3 29l3 4 5 2 12-8a619 619 0 0 0 32-27 177 177 0 0 0 23-31l-3-12c-3-8-3-10-2-11l8-4c6-3 7-3 16-3s10 0 17 4c6 2 10 5 14 9l8 11 3 12v11l-2 7-12 1-11 1c-1 0-4 2-5 5l-8 11a202 202 0 0 0-28 69l48 31a43 43 0 0 0 26 0l10-6 6-4 2 2c1 2 0 3-1 7a158 158 0 0 1-11 21l-9 11-12 9c-4 2-9 4-14 4h-16l-15-5-143-99-10-5-10-2c-5-1-8 0-13 1l-10 3-6 6a9106 9106 0 0 0-58 266c1 1 110-249 111-249l44 30-71 127 96-109 43 31c1 1-233 209-234 209L115 491l43-33 98 111c1 1-73-128-72-130 1-1 44-30 45-29 0 1 106 251 108 249 1-1-54-262-55-263l-4-5c-1-2-5-4-8-5-4-2-7-3-14-3l-13 2-12 4-22 15-53 37-41 30a182 182 0 0 0-13 9l-11 7-18 7a48 48 0 0 1-25-2l-12-7a69 69 0 0 1-20-26l-6-11c-2-5-2-5 2-9l3 3 9 6c4 3 7 4 13 4l12-1a80 80 0 0 0 21-9l33-23-7-20a211 211 0 0 0-33-63c-2-3-3-3-10-3H42l-4-1-1-6-2-8 2-8 3-9 4-7a53 53 0 0 1 22-15c5-2 8-2 14-2l15 4c4 2 7 4 7 6l-3 11c-2 8-3 11-2 12a161 161 0 0 0 18 24 141 141 0 0 0 29 26l20 15 3-2 5-4c1-1 0-4-1-10a263 263 0 0 1-2-41l3-19a103 103 0 0 1 13-29 170 170 0 0 1 19-21 281 281 0 0 1 27-16 219 219 0 0 1 76-19l8-95zm4 153a423 423 0 0 0-49 7 237 237 0 0 0-39 15 90 90 0 0 0-30 27 87 87 0 0 0-11 23l-2 9c0 2 1 3 2 3l6-3 11-6 13-5c6-2 9-2 16-2 7 1 10 1 16 5 6 2 9 5 15 11 7 7 8 8 7 5l-12-62 9-4 17-4a188 188 0 0 1 41 0l20 187 18-185c1-3 1-3 8-4a156 156 0 0 1 46 5l14 4a3149 3149 0 0 0-11 65l5-5 11-10 12-6 13-2a58 58 0 0 1 25 6l12 7c3 2 6 4 7 3v-6l-2-11-5-12-9-13-13-12a137 137 0 0 0-54-24 235 235 0 0 0-36-5 524 524 0 0 0-71-1z"/>
    </svg>
"#;

// -----------------------------------
// NAVBAR
// -----------------------------------

pub const CAMPFIRE: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
      <path fill-rule="evenodd" d="M301 17c-1 13 8 35 17 47 13 17 44 65 47 71 8 15 14 37 15 43 8 36 1 63-4 80a128 128 0 0 1-172 72c-35-15-68-55-72-84-4-36-7-48 2-83 6-20 19-37 24-42s13-13 19-9c10 6 8 24 15 35 9 15 20 21 20 21l3-31c3-25 11-54 20-74 15-30 36-48 43-55 9-10 9-8 15-8 2 0 9 5 8 17zM71 445c8-16 28-21 28-21s-27-11-32-30c-7-26 5-44 15-50 21-13 37-7 37-7l137 40 68-20 74-21s20-3 35 11c12 11 17 26 14 43-6 25-33 34-33 34s27 10 32 31c6 25-5 35-10 42-14 19-44 13-44 13l-136-40-139 41s-13 5-31-5c-17-9-29-35-15-61zm15 25c0 11 11 20 21 20 9 0 21-9 21-21s-10-21-22-21-20 10-20 22zm298 0c0 13 12 20 22 20s20-11 20-21c-1-11-10-21-21-21-10 0-21 8-21 22z"/>
    </svg>
"#;
pub const BOOK: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path d="M61 360c3 5 7 14 28 12 13-1 53 5 83 16 39 13 69 43 69 43V141c-1-3-18-33-56-56C141 59 80 59 72 61c-7 2-10 9-11 12l-1 287zm-16 26c-10-10-12-20-12-20l-1-242-21 2-6 4c-1 1-3 3-3 5-2 4 0 306 0 306s1 4 8 8 15 2 18 1c2 0 51-9 82-11 38-3 104 7 104 7s-46-41-95-43c-17 0-43-4-53-5-9-1-17-8-21-12zm407-26c-3 5-7 14-28 12-13-1-53 5-83 16-39 13-69 43-70 43l1-290c0-3 18-33 56-56 44-26 105-26 113-24 7 2 9 9 10 12l1 287zm16 26c10-10 12-20 12-20l1-242 21 2 6 4 3 5v306s-2 4-8 8c-7 4-16 2-18 1-2 0-51-9-82-11-39-3-104 7-104 7s46-41 95-43c17 0 43-4 53-5 9-1 16-8 21-12z"/>
    </svg>
"#;
pub const HELM: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path d="m251 9-33 18c-2 1 12 109 12 111l3 5h45c2 0 3-2 3-3 1-1 15-111 14-113l-5-4-35-15-4 1zm-67 53a172 172 0 0 0-31 19 204 204 0 0 0-36 38 193 193 0 0 0-17 34l-6 18-2 11-1 171-26 33c0 1 123 118 125 118l4-2c2-2 1-203 1-203l-53-32v-47l83 48 2 72 29 21 29-21 2-73 83-47v48l-52 30s-2 202 0 204l4 2c2 0 125-117 125-117l-25-34-2-172a309 309 0 0 0-15-46l-11-19a174 174 0 0 0-68-55l-8-3-12 80c0 5-2 10-3 12l-4 7c-2 3-6 5-10 7l-7 3c-52 0-53 0-58-2l-8-5-6-8c-2-3-3-8-4-14-1-5-10-80-12-80l-10 4z" />
    </svg>
"#;
pub const BACKPACK: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path fill-rule="evenodd" d="M91 0h30v30h270V0h30v30h9l15 1 10 3 10 7 9 10 5 11c2 6 2 81 1 85l-4 10-8 11-12 8-11 4-24 1V30h-30v151H121V30H91v151h-9l-15-1c-3-1-8-2-10-4a47 47 0 0 1-24-27c-2-6-2-81-1-85l4-10a52 52 0 0 1 20-19l11-4 24-1zm0 211h330v301h-60l-1-145-4-11a51 51 0 0 0-20-20l-11-4H187l-11 4-12 8a44 44 0 0 0-12 23l-1 145H91zM60 334v145l-5-3a47 47 0 0 1-18-18l-5-10v-82l4-11 8-11a51 51 0 0 1 16-10zm392 0a46 46 0 0 1 24 21l4 10v83l-4 10-8 11a49 49 0 0 1-16 10V334zm-129 29q4 2 6 6c2 4 2 6 2 143H181s0-139 2-143c1-2 3-5 6-6 3-2 130-2 134 0z" />
    </svg>
"#;

// -----------------------------------
// CUSTOM
// -----------------------------------

pub const EXIT: &str = r#"
    <svg viewBox="0 0 48 48">
        <path fill-rule="evenodd" d="M26 13h-6V7H6v34h14v-6h6v12H0V1h26zm9 21h-3v-2l7-6H18l-2-2 2-2h21l-7-6v-2h3l11 9v2l-11 9z"/>
    </svg>
"#;
pub const BELL: &str = r#"
    <svg viewBox="0 0 512 512">
    	<path d="m245.7 3c-2.6 1.7-5.9 4.9-7.4 7.2-2.7 4-2.8 4.9-3.3 19.2l-0.5 15.1c-3.9 0.5-9.9 1.9-16 3.5-6 1.5-15.1 4.5-20 6.7-4.9 2.1-12.4 5.7-16.5 8.1-4.1 2.3-10.6 6.6-14.5 9.4-3.8 2.8-11.7 9.9-17.5 15.7-6.4 6.5-13.2 14.5-17.2 20.6-3.7 5.5-8.7 14.3-11.2 19.5-2.4 5.2-5.6 12.9-7 17-1.3 4.1-3.5 12-4.7 17.5-2 9.2-2.2 14-2.9 59-0.7 44.2-1 49.9-2.8 58.5-1.2 5.2-3.5 13.6-5.3 18.5-1.7 4.9-4.9 12.6-7.2 17-2.2 4.4-5.1 9.8-6.6 12-1.4 2.2-5.3 7.6-8.7 12-3.4 4.4-10.8 12.5-16.5 18-8 7.8-10.9 11.4-13.3 16.5-2.7 5.7-3.1 7.5-3.1 15.5 0 8.1 0.3 9.7 3.2 15.5 2.1 4.4 5 8.2 9 11.8 3.3 3.1 8.1 6.3 11.3 7.5l5.5 2.2c311.3 0.5 361.6 0.3 366-0.5 3.3-0.7 8.6-2.6 11.9-4.3 3.2-1.8 7.4-4.9 9.2-7 1.9-2 4.5-5.8 5.7-8.5 1.3-2.6 3-6.7 3.6-9.2 0.7-2.5 1.1-7 0.8-10-0.2-3-1.5-8.2-2.7-11.5-2-5.1-4.1-7.8-14.4-18.1-6.7-6.6-14.6-15.4-17.7-19.5-3.1-4.1-7.4-10.5-9.7-14.4-2.3-3.9-5.7-10.6-7.6-15-1.9-4.4-4.6-12.1-6.1-17-1.4-4.9-3.2-13.1-4-18-1.1-6.7-1.5-18.6-1.5-47 0-20.9-0.5-42.3-1-47.5-0.6-5.2-1.7-13.1-2.6-17.5-0.9-4.4-2.7-11.4-4.1-15.5-1.3-4.1-5-12.9-8.2-19.5-3.3-6.6-8.8-16-12.2-20.9-3.5-5-10.5-13.1-15.4-18.1-4.9-5-12.4-11.6-16.5-14.6-4.1-3.1-11.1-7.7-15.5-10.2-4.4-2.6-12.3-6.4-17.5-8.6-5.2-2.1-14.2-5-20-6.5-5.8-1.4-11.5-2.6-12.7-2.6-2.3 0-2.3-0.2-2.6-14.8-0.2-13.8-0.4-15-2.8-19.2-1.6-2.8-4.5-5.7-7.8-7.8-4.2-2.6-6.1-3.2-10.9-3.2-4.6 0-6.6 0.6-10.5 3zm-67.7 445.3c0 0.1 0.9 3.3 2.1 7.2 1.2 3.9 3.3 9.2 4.6 12 1.4 2.7 4.4 7.7 6.8 11 2.3 3.3 7.4 8.9 11.3 12.5 4.2 3.9 10.6 8.4 15.9 11.2 4.8 2.5 13 5.8 18.1 7.2 7.2 2 11.5 2.6 19.7 2.6 7.5 0 12.7-0.6 18.3-2.1 4.2-1.1 11.8-4 16.7-6.4 4.9-2.4 11.9-6.8 15.4-9.7 3.6-2.9 8.7-8 11.3-11.3 2.7-3.3 6.9-10.1 9.3-15 2.4-5 5.1-11.4 6-14.3l1.5-5.2c-121.7 0-157 0.1-157 0.2z"/>
    </svg>
"#;
pub const FIST: &str = r#"
    <svg viewBox="0 0 512 512">
        <path fill-rule="evenodd" d="m211 0 57 32c0 1-35 60-39 64l-59-28 38-67 3-1zm-60 74 124 59s-3 29-34 38c-33 9-45-5-54-9l-16 32c0 1 36 16 66 49 33 38 50 105 50 104l17-82c1 0 13 5 55 27l65-67-3 39c0 3-14 32-60 124l-1 123H165l21-143L72 195zm121 336 4 77 16-75 26-20h-43l-30-9zm-31-310 40-66 62 39a4715 4715 0 0 1-62 92c0-1 2-38 0-40-1-2-40-24-40-25zm78 129-3-1-39-37c0-1 75-103 77-103l49 47s-71 81-84 94zm36 39c-2 0-27-30-26-32l77-84c2 0 34 41 34 42 0 2-82 75-85 74z"/>
    </svg>
"#;
pub const BULLSEYE: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path fill-rule="evenodd" d="M256 512A256 256 0 1 1 366 25l-38 38A207 207 0 0 0 50 256a206 206 0 1 0 401-67l40-35a257 257 0 0 1-235 358zm0-100a156 156 0 1 1 67-297l-37 39a106 106 0 1 0 73 77l39-39a156 156 0 0 1-142 220zm0-100a56 56 0 1 1 23-107l64-72 5-58 71-74v68l24 23h67l-73 74-59 5-72 61c4 7 6 15 6 24 0 31-25 56-56 56z"/>
    </svg>
"#;
pub const HEART: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width="30">
    	<path d="m267.8 490.4l166.9-187.5q61.3-71.6 61.3-140.6 0-68.8-34-107.6-34-38.7-94-38.7-16.6 0-33.9 6.7-17.3 6.7-32.2 18.1-14.8 11.5-25.5 21.4-10.8 10.1-20.4 21.3-9.6-11.2-20.4-21.3-10.7-9.9-25.5-21.4-14.9-11.4-32.2-18.1-17.3-6.7-33.9-6.7-60 0-94 38.7-34 38.8-34 107.6 0 20.9 6.3 43.1 6.3 22.2 14.3 37.8 8.1 15.6 18.2 30.5 10.2 14.8 14.9 20.4 4.7 5.7 7.4 8.2l167.1 188.1q4.8 5.6 11.8 5.6 7 0 11.8-5.6z"/>
    </svg>
"#;
pub const SHIELD: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path d="M45 80s-3 146 13 221a275 275 0 0 0 195 210l3 1 4-1s65-23 90-44c44-37 76-57 103-162 21-84 14-225 14-225s-75-1-126-20c-52-21-85-60-85-60s-42 43-96 63C109 83 45 80 45 80z"/>
    </svg>
"#;
pub const SHIELD_BROKEN: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
        <path fill-rule="evenodd" d="M160 63c13-5 26-11 37-18l19 17-29 44 46 47-40 86 81 172-25-181 60-79-38-48 52-51 18 8c51 19 126 20 126 20s7 141-14 225c-27 105-59 125-103 162-25 21-90 44-90 44l-4 1-3-1A276 276 0 0 1 58 301C42 226 45 80 45 80s64 3 115-17z"/>
    </svg>
"#;
pub const DIE: &str = r#"
    <svg viewBox="0 0 512 512" stroke-width=0>
    	<path fill-rule="evenodd" d="m450 98.8l11.4 9.1 9.7 10.4 3.9 5.7-204.5 128.7-10 1.3-9.5-1.3-211-132.7 7.2-8.6 13.8-11.9 159-91.4 15.5-5.7 19.7-2.3h0.1c8.8-0.1 19.4 1.9 19.4 1.9l10.8 3.2 9.5 4.3zm-208.7-14.9l-8.8 6.4-7.2 9.7-2.7 14.5 2.7 14.5 7.2 9.8 8.3 6.1 9.5 3h11.5l9.5-3 8.4-6.1 7.2-9.8 2.7-14.5-2.7-14.5-7.5-10-9.8-6.6-10.5-2.8-9.5 0.3zm12.3 183l0.5 245.1h-4.5c-2.5 0-8-0.9-12.2-2-4.3-1.1-10.7-3.2-14.3-4.7-3.6-1.5-162.5-93-165-94.8-2.5-1.8-7.7-6.6-11.5-10.6-3.9-4.1-9-10.5-11.3-14.4-2.3-3.8-5.5-10.6-7.1-15-1.6-4.4-3.6-12.5-4.6-18-1.4-8.9-0.7-192.8 0.7-198.5 0.9-3.6 2.8-9.6 4.3-13.5 1.4-3.8 2.9-7.4 3.2-7.8 0.3-0.5 1.1-0.6 1.7-0.4 0.6 0.2 208.9 130.8 220.1 134.6zm-92.6 3.2l-4 7.2-0.6 12.7 3.8 17.5 7.4 14.9 8.5 10.4 8.5 6.4 8 2 6.5-2 4.2-4.8 2.4-9.4-1-14.5-6.1-17-10.2-15.4-9.8-8.6-7-2.7c0 0-4.1-0.3-5.4-0.1-1.3 0.3-5.2 3.4-5.2 3.4zm-106.9 52.9l-2.6 9.3 1 13.7 6.5 18.5 10.7 16.2 10.4 8.2 8.3 2.1 7-2.9 4.4-6.7 1.3-10.9-1.5-13-5.9-15-9.8-15-10.9-8.8-8.4-2.7c0 0-4.2 0.7-5.7 1.5-1.6 0.8-4.8 5.5-4.8 5.5zm428.4-185.2c0.5 0.9 1.7 4 2.6 6.7 1 2.8 2.5 9.5 3.4 15 1.5 9 0.8 192.8-0.6 198.5-0.9 3.6-2.8 9.6-4.3 13.5-1.4 3.9-4.5 10.1-6.8 14-2.2 3.9-7.3 10.3-11.2 14.4-3.8 4-167.4 101.1-171.5 103.2-4.1 2.1-11.1 4.7-15.5 5.8-4.4 1.2-8.8 2.1-9.7 2.1-1.7 0-1.8-6.5-1.9-243.5 0 0 214.6-131.3 215.5-129.7zm-47.6 62l-9.5 11.3-7.3 14.2-4.1 15.5 0.1 13.7 4.6 9.3 7.9 3.2 7.8-1.6 6.6-4.3 7.9-8.6 7.8-13 4.8-13 1.6-14-2-12.2-4.2-5.7c0 0-3.4-1.6-4.8-1.9-1.4-0.3-7.2 0.9-7.2 0.9zm-108.8 70l-6 4.4-6.2 6.8-7.3 12-5.8 15.5-1.5 15 2.3 10.8 4.5 5.4 7 1.4 8.5-2.5 9.5-8.3 9.2-12.8 5.7-13.5 2.8-14-0.6-11.7-4-7.5-5.7-3.8c0 0-4.3-0.3-5.9 0-1.7 0.3-6.5 2.8-6.5 2.8zm108.8 54l-9.5 11.3-7.3 14.2-4.1 15.5 0.1 13.7 4.6 9.3 7.9 3.2 7.8-1.6 6.6-4.3 7.9-8.6 7.8-13 4.8-13 1.6-14-2-12.3-4.2-5.6c0 0-3.4-1.7-4.8-2-1.4-0.3-7.2 0.9-7.2 0.9zm-107.8 69.2l-6 4.3-7.8 8.6-7.2 11.6-5 14-2 14 1.3 9.9 4 6.8 7.5 2.8 8.7-1.9 10-8.3 9.7-13.3 5.9-14 2.4-16.5-2.3-13.5-4-5.1-4.7-1.7c0 0-4.1-0.2-5.5 0.2-1.4 0.4-5 2.1-5 2.1z"/>
    </svg>
"#;
pub const HEALING: &str = r#"
    <svg viewBox="0 0 443 443">
		<path d="m239 23v14h-29v37h29v29h37v-29h29l-0.5-36.5-28.5-0.5v-28h-37zm111.8 103l-0.3 19.5-39.5 0.5 0.5 50.5 39.5 0.5c0 29.4 0.3 38.2 0.8 38.4 0.4 0.3 3.8 0.6 7.7 0.6 3.9 0.1 14.8 0.2 24.3 0.1l17.2-0.1v-39l39.5-0.5 0.5-50.5-39.5-0.5-1-39h-49.5zm-235.2 58.8v57.7l-116.5 0.5v83h117l-0.5 116c76.1 0 81.5-0.2 82-1.5 0.2-0.8 0.5-25.8 0.5-55.5 0-29.7 0-55.1 0-56.5v-2.5l116.5-0.5c0.4-61.6 0.7-79.9 1-80.5 0.2-0.6 0.1-1.2-0.3-1.5-0.4-0.2-26.4-0.4-57.7-0.5-31.4 0-57.6-0.4-58.3-0.8-1-0.5-1.3-4.1-1.3-14.4 0-7.6 0.3-14.2 0.6-14.8 0.2-0.6 0.1-1-0.3-1-0.4 0-0.7-19.1-0.6-42.5l0.1-42.5h-82.2z"/>
    </svg>
"#;
pub const WEIGHT: &str = r#"
    <svg viewBox="0 0 128 128" stroke-width="0">
        <path fill-rule="evenodd" d="m12 128-7-5-3-8 12-70 3-6 8-4h19s-5-6-3-18c2-7 11-17 23-17s21 9 22 17c3 10-1 18-1 18l20 1 6 3 3 6 11 69s0 5-3 9c-2 4-7 5-7 5zM54 24c0 5 5 9 10 9s10-4 10-9c0-4-3-10-10-10-5 0-10 4-10 10z"/>
    </svg>
"#;

// -----------------------------------
// MISC
// -----------------------------------

pub const CODEBERG: &str = r#"
    <svg viewBox="0 0 33 33">
	<defs>
		<linearGradient id="g1" x2="1" gradientUnits="userSpaceOnUse" gradientTransform="matrix(10.365,20.686,-8.127,4.072,16.785,8.513)">
			<stop offset="0" stop-color='#ffffff' stop-opacity="0"/>
			<stop offset=".495" stop-color='#71c2ff' stop-opacity="1"/>
			<stop offset="1" stop-color='#39aaff' stop-opacity="1"/>
		</linearGradient>
	</defs>
	<path style="opacity: .5; fill: url(#g1)" d="m16.8 8.5q0 0-0.1 0 0 0 0 0.1 0 0 0 0-0.1 0-0.1 0l6.3 23.3q1.1-0.5 2.1-1.1 1-0.7 2-1.5 0.9-0.7 1.7-1.7 0.8-0.9 1.4-1.9l-13.2-17.1q0 0 0-0.1 0 0 0 0-0.1 0-0.1 0 0 0 0 0z"/>
	<path style="fill: #2185d0; stroke: none" d="m16.4 1c-2.1 0-4.2 0.4-6.1 1.2-2 0.8-3.7 2-5.2 3.5-1.5 1.5-2.7 3.3-3.5 5.3-0.8 1.9-1.2 4-1.2 6.1q0 1.1 0.1 2.3 0.2 1.1 0.5 2.2 0.3 1.1 0.8 2.1 0.4 1 1 2l13.5-17.4q0 0 0.1-0.1 0 0 0.1 0 0 0 0.1 0 0.1 0.1 0.1 0.1l13.4 17.4q0.6-1 1.1-2 0.5-1 0.8-2.1 0.3-1.1 0.4-2.2 0.2-1.2 0.2-2.3c0-2.1-0.4-4.2-1.2-6.2-0.8-1.9-2-3.7-3.5-5.2-1.5-1.5-3.3-2.7-5.2-3.5-2-0.8-4.1-1.2-6.2-1.2q0 0 0 0 0 0 0 0-0.1 0-0.1 0 0 0 0 0z"/>
</svg>
"#;
