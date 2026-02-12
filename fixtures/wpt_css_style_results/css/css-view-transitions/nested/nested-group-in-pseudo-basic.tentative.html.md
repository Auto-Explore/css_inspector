# css/css-view-transitions/nested/nested-group-in-pseudo-basic.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-group-in-pseudo-basic.tentative.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    main {
        view-transition-name: main;
    }

    section {
        view-transition-name: section;
        view-transition-group: main;
    }

    ::view-transition,
    ::view-transition-group(*),
    section,
    main {
        background: red;
        inset: 0;
        position: absolute;
    }

    ::view-transition-group(main) {
        background: green;
        animation-play-state: paused;
    }
    ::view-transition-group(section) {
        background: inherit;
        transform: none !important;
    }
    ::view-transition-image-pair(*),
    ::view-transition-old(*),
    ::view-transition-new(*)
     {
        display: none;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
