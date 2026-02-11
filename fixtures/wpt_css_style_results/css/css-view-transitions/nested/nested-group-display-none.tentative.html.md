# css/css-view-transitions/nested/nested-group-display-none.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-group-display-none.tentative.html"
}
```

## style[0]

```css

    body {
        background: green;
    }
    main {
        view-transition-name: main;
    }

    section {
        view-transition-name: section;
        view-transition-group: main;
    }

    ::view-transition-group(*)
    ::view-transition-image-pair(*),
    ::view-transition-old(*),
    ::view-transition-new(*)
     {
        display: none;
    }
```

```json
{
  "errors": 4,
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
      "message": "Unknown property “view-transition-group”.",
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
