# css/css-view-transitions/unset-and-initial-view-transition-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/unset-and-initial-view-transition-name.html"
}
```

## style[0]

```css

#first {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: target;
}
#second {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: unset;
}
#third {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: initial;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
