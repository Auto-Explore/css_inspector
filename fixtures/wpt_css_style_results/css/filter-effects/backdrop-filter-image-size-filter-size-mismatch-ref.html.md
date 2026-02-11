# css/filter-effects/backdrop-filter-image-size-filter-size-mismatch-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-image-size-filter-size-mismatch-ref.html"
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
        position: relative;
    }

    div#filter {
        position: absolute;
        /* although the blur only technically requires an extra radius of <blur radius>px,
            most implementations have quite different effects if the backdrop size is ~2x
            larger aroud a harsh color boundary, so to ease flakes, some buffer is added.
            This shouldn't affect this test's ability to discriminate */
        left: -25px;
        width: 250px;
        height: 200px;
        backdrop-filter: blur(10px);
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
