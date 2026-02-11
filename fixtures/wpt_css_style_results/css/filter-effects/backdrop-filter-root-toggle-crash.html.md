# css/filter-effects/backdrop-filter-root-toggle-crash.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-root-toggle-crash.html"
}
```

## style[0]

```css

    #container {
      view-transition-name: arandomname;
      width: 100px;
      height: 100px;
      background: blue;
    }

    #overlay {
      width: 100%;
      height: 100%;
    }

    #overlay.blur {
      backdrop-filter: blur(8px);
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
