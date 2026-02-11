# css/cssom/border-shorthand-serialization.html

```json
{
  "format_version": 3,
  "file": "css/cssom/border-shorthand-serialization.html"
}
```

## style[0]

```css

    .a {
        border-width: 1px;
        border-style: solid;
        border-color: black;
    }
    .b {
        border-width: 1px;
        border-style: solid;
        border-color: black;
        border-image: linear-gradient(white,black);
    }
    .c {
        border: 1px solid black;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
