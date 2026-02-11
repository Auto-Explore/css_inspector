# css/css-backgrounds/background-clip/clip-border-area-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/background-clip/clip-border-area-ref.html"
}
```

## style[0]

```css

    .test {
        display: inline-block;
        margin: 20px;
        width: 300px;
        height: 150px;
        box-sizing: border-box;
        border: 50px solid blue;
    }

    .rounded {
        border-radius: 40%;
    }

    .missing-border {
        border-right-style: hidden;
    }

    .double {
        border-style: double;
    }

    .inset {
        border-style: solid;
    }

    .complex {
        border-right-style: double;
        border-bottom-style: dashed;
        border-left-style: dotted;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
