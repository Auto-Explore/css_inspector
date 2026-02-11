# css/css-pseudo/highlight-cascade/highlight-paired-cascade-004.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-paired-cascade-004.html"
}
```

## style[0]

```css

    :link, :visited {
        color: blue;
    }
    main {
        font-size: 7em;
        margin: 0.5em;
    }
    main::target-text {
        /*
            Used background-color should be initial (transparent).
            https://www.w3.org/TR/CSS21/colors.html#propdef-background-color
        */
        color: black;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
