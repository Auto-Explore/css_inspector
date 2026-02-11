# css/css-gaps/grid/grid-gap-decorations-045.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-045.html"
}
```

## style[0]

```css

    #container,
    body {
        overflow: hidden;
        margin: 0px;
    }

    #container {
        display: grid;
        grid-template-rows: 4rem 2rem auto;
        width: 50%;
        gap: 2px;
        border: solid 2px black;
        row-rule: 2px solid hotpink, 1px dashed grey;
        background-color: teal;
    }

    #one {
        height: 100px;
        width: 100px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “row-rule”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
