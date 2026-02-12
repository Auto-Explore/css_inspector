# css/css-animations/animation-composition.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-composition.html"
}
```

## style[0]

```css

    @keyframes anim {
        from {
            filter: blur(10px);
            width: 100px;
        }
        50% {
            filter: blur(15px);
            width: 228px;
        }
        to {
            filter: blur(20px);
            width: 1337px;
        }
    }

    .anim-target {
        animation: anim 1s;
        animation-fill-mode: forwards;
        animation-timing-function: linear;
        filter: blur(5px);
        width: 50px;
    }

    .replace {
        animation-composition: replace;
    }

    .add {
        animation-composition: add;
    }

    .accumulate {
        animation-composition: accumulate;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
