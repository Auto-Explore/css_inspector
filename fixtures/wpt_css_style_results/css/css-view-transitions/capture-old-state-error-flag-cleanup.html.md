# css/css-view-transitions/capture-old-state-error-flag-cleanup.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/capture-old-state-error-flag-cleanup.html"
}
```

## style[0]

```css

        #a,
        #b {
            width: 100px;
            height: 50px;
            background-color: green;
            view-transition-name: foo;
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
