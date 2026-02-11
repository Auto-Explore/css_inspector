# css/css-display/display-contents-before-after-002.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-before-after-002.html"
}
```

## style[0]

```css

    /* Disable kerning because kerning may differ for different node tree. */
    html { font-kerning: none; font-feature-settings: "kern" off; }
    div::before {
        display: contents;
        border: 100px solid red;
        content: "P"
    }
    div::after {
        display: contents;
        border: 100px solid red;
        content: "S"
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
