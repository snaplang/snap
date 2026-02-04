# Lists and Matrices

Snap supports lists (arrays) and matrices (2D arrays) as first-class data types. Lists in Snap use **0-based indexing**, which is automatically converted to Scratch's 1-based indexing during compilation.

## Declaring Lists

Lists are declared using the `list<T>` type, where `T` is the element type:

```snap
let scores: list<int> = [10, 20, 30, 40, 50];
let names: list<string> = ["Alice", "Bob", "Charlie"];
let flags: list<bool> = [true, false, true];
```

## Declaring Matrices

Matrices are declared using the `matrix<T>` type:

```snap
let grid: matrix<int> = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
];
```

> **Note:** Matrices are stored internally as flattened lists in Scratch, since Scratch doesn't have native matrix support.

## Accessing Elements

### List Index Access

Access list elements using bracket notation with 0-based indices:

```snap
let items: list<int> = [10, 20, 30];

on GreenFlag {
    // Access first element (index 0)
    looks::Say(items[0]);  // Says "10"

    // Access last element
    looks::Say(items[2]);  // Says "30"
}
```

### Matrix Index Access

Access matrix elements using comma-separated indices `[x, y]`:

```snap
let grid: matrix<int> = [[1, 2], [3, 4]];

on GreenFlag {
    // Access element at x=1, y=0 (column 1, row 0)
    looks::Say(grid[1, 0]);  // Says "2"
}
```

## List Operations

### Adding Items

Add items to the end of a list:

```snap
let items: list<int> = [];

on GreenFlag {
    items.add(10);
    items.add(20);
    items.add(30);
    // items is now [10, 20, 30]
}
```

### Deleting Items

Delete items by index or delete all items:

```snap
let items: list<int> = [10, 20, 30, 40];

on GreenFlag {
    items.delete(0);      // Delete first item (index 0)
    items.delete(last);   // Delete last item
    items.delete(random); // Delete random item
    items.delete(all);    // Delete all items
}
```

### Inserting Items

Insert an item at a specific index:

```snap
let items: list<int> = [10, 30];

on GreenFlag {
    items.insert(1, 20);  // Insert 20 at index 1
    // items is now [10, 20, 30]
}
```

### Replacing Items

Replace an item at a specific index:

```snap
let items: list<int> = [10, 20, 30];

on GreenFlag {
    items.replace(1, 25);  // Replace item at index 1 with 25
    // items is now [10, 25, 30]

    // Alternative syntax using bracket assignment:
    items[2] = 35;
    // items is now [10, 25, 35]
}
```

## List Reporters

### Length

Get the number of items in a list:

```snap
let items: list<int> = [10, 20, 30];

on GreenFlag {
    looks::Say(items.length());  // Says "3"
}
```

### Contains

Check if a list contains a specific item:

```snap
let items: list<int> = [10, 20, 30];

on GreenFlag {
    if items.contains(20) {
        looks::Say("Found it!");
    }
}
```

### Index Of

Find the index of an item (returns 0-based index, or -1 if not found):

```snap
let items: list<string> = ["apple", "banana", "cherry"];

on GreenFlag {
    let idx: int = items.index_of("banana");
    looks::Say(idx);  // Says "1"
}
```

## Example: High Score List

```snap
let high_scores: list<int> = [100, 80, 60, 40, 20];

new Sprite("ScoreManager") {
    implements Code {
        fn add_score(score: int) {
            // Find the right position to insert
            let i: int = 0;
            control::RepeatUntil(i >= high_scores.length() || score > high_scores[i]) {
                change i by 1;
            }

            // Insert the new score
            high_scores.insert(i, score);

            // Keep only top 5 scores
            if high_scores.length() > 5 {
                high_scores.delete(5);
            }
        }

        on GreenFlag {
            add_score(75);  // Insert a new score
        }
    }
}
```

## Example: Grid-Based Game

```snap
let game_grid: matrix<int> = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0]
];

new Sprite("Player") {
    position: (0, 0),

    implements Code {
        let player_x: int = 1;
        let player_y: int = 1;

        on GreenFlag {
            // Mark player position using [x, y] syntax
            game_grid[player_x, player_y] = 1;
        }

        on KeyPressed("right") {
            if player_x < 2 {
                game_grid[player_x, player_y] = 0;
                change player_x by 1;
                game_grid[player_x, player_y] = 1;
            }
        }
    }
}
```

## Scratch Block Mapping

| Snap Operation              | Scratch Block                                |
| --------------------------- | -------------------------------------------- |
| `list.add(item)`            | `add [item] to [list]`                       |
| `list.delete(index)`        | `delete [index] of [list]`                   |
| `list.insert(index, item)`  | `insert [item] at [index] of [list]`         |
| `list.replace(index, item)` | `replace item [index] of [list] with [item]` |
| `list[index]`               | `item [index] of [list]`                     |
| `matrix[x, y]`              | `item [x + y * width + 1] of [matrix]`       |
| `list.length()`             | `length of [list]`                           |
| `list.contains(item)`       | `[list] contains [item]?`                    |
| `list.index_of(item)`       | `item # of [item] in [list]`                 |

## Important Notes

1. **0-Based Indexing**: Snap uses 0-based indexing (like most programming languages), but Scratch uses 1-based indexing. The compiler automatically handles this conversion.

2. **Matrix Storage**: Matrices are stored as flattened lists in Scratch. A 3x3 matrix is stored as a list with 9 elements in row-major order.

3. **Type Safety**: While Snap enforces types at the language level, Scratch lists can hold mixed types. The type annotations help catch errors during development.

4. **Performance**: List operations in Scratch can be slow for large lists. Consider using smaller lists or optimizing algorithms for better performance.
