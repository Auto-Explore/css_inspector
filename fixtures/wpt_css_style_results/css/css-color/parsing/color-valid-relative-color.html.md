# css/css-color/parsing/color-valid-relative-color.html

```json
{
  "format_version": 3,
  "file": "css/css-color/parsing/color-valid-relative-color.html"
}
```

## style[0]

```css

    html {
        --bg-color: blue;
        --color: green;
        --accent: lightseagreen;
        --mycolor: orchid;
        --mygray: lch(from var(--mycolor) l 0 h);
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
