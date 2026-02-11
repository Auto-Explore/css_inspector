# css/filter-effects/backdrop-filter-image-size-filter-size-mismatch.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-image-size-filter-size-mismatch.html"
}
```

## style[0]

```css

    div#outer {
        display: inline-block;
        will-change: backdrop-filter;
    }

    div#inner {
        width: 200px;
        height: 200px;
        background-color: blue;
    }

    div#filter {
        width: 200%;
        height: 100%;
        backdrop-filter: blur(10px);
        transform: translate(-25%, 0);
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
