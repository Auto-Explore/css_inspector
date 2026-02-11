# css/css-display/display-contents-details.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-details.html"
}
```

## style[0]

```css

  html { font-kerning: none; font-feature-settings: "kern" off; }
  details, summary {
    all: initial;
    border: 10px solid red;
    display: contents;
  }
  details::details-content {
    display: contents;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
