# css/css-view-transitions/navigation/root-element-transition-opt-in-removed-on-old.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/root-element-transition-opt-in-removed-on-old.html"
}
```

## style[0]

```css

@view-transition { navigation: auto; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

html {
  background: blue;
}
.hidden {
  width: 10px;
  height: 10px;
  view-transition-name: hidden;
  background: green;
  contain: layout;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
