# css/css-view-transitions/navigation/opt-in-removed-during-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/opt-in-removed-during-transition.html"
}
```

## style[0]

```css

  @view-transition {
    navigation: auto;
  }
  #target {
    view-transition-name: target;
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
