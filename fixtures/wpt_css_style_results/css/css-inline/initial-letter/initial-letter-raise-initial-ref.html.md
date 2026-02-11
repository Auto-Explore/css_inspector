# css/css-inline/initial-letter/initial-letter-raise-initial-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/initial-letter-raise-initial-ref.html"
}
```

## style[0]

```css

    .sample {
        border: solid 1px green;
        font-family: Ahem;
        font-size: 20px;
        line-height: 24px;
        width: 230px;
    }

    .fake-initial-letter {
        background: lime;
        float: left;
        --cap: 0.8;
        --size: calc((24px * 2 + 20px * var(--cap)) / var(--cap));
        height: var(--size);
        width: var(--size);
        margin-top: 2px;
    }

    .no-ascent .fake-initial-letter {
        height: calc(var(--size) * (1 - var(--cap)));
        margin-top: calc(var(--size) * var(--cap) + 2px);
    }
    .no-descent .fake-initial-letter {
        height: calc(var(--size) * var(--cap));
    }
    .rtl {
        direction: rtl;
        .fake-initial-letter {
            float: right;
        }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
