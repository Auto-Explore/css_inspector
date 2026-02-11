# css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-033.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/contain-intrinsic-size/contain-intrinsic-size-033.html"
}
```

## style[0]

```css

    .test {
        width: max-content;
        height: max-content;
        border: 1px solid;
    }

    .test::before {
        content: "";
        display: block;
        width: 320px;
        height: 240px;
    }

    .contain-size {
        contain: size;
    }

    .ci-width {
        contain-intrinsic-width: 10px;
    }

    .ci-height {
        contain-intrinsic-height: 20px;
    }

    .ci-both {
        contain-intrinsic-size: 10px 20px;
    }

    .skip-contents .test {
        content-visibility: hidden;
    }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
