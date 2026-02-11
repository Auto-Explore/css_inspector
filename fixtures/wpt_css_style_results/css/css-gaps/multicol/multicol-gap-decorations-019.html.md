# css/css-gaps/multicol/multicol-gap-decorations-019.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/multicol/multicol-gap-decorations-019.html"
}
```

## style[0]

```css

    body {
        margin: 0px;
    }

    #current {
        color: firebrick;
        columns: 6;
        column-gap: 2px;
        column-rule-style: solid;
        column-rule-width: 2px;
        column-fill: auto;
        height: 20px;
        column-rule-color: repeat(auto, currentColor);

        width: 72px;
        height: 20px;
    }

    .items {
        background-color: lightgreen;
        height: 20px
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
