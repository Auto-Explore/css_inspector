# css/css-sizing/intrinsic-percent-non-replaced-003.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/intrinsic-percent-non-replaced-003.html"
}
```

## style[0]

```css

  /* establish context */
  .container {
    clear: both;
    padding: 10px;
    color: blue;
    font: 20px/1 Ahem;
  }
  .zero {
    width: 0;
  }
  .infinite {
    width: 400px; /* close enough */
  }

  /* visualize size contribution */
  .container > div {
    float: left;
    border: solid orange 20px;
    border-style: none solid;
  }
  .container > div > div {
    border-right: solid 20px aqua;
  }

  /* test width */
  /* content = 100% = 80px = 4ch + border */
  /* choose sizes that are different than content to see if how they take effect */
  .raw-percent {
    width: 50%;
  }
  .calc-percent {
    width: calc(40px + 0%);
  }
  .no-percent {
    width: 40px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
