# css/css-sizing/intrinsic-percent-non-replaced-005.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/intrinsic-percent-non-replaced-005.html"
}
```

## style[0]

```css

  /* establish context */
  .container {
    clear: both;
    padding: 10px;
    width: 0;
  }

  span {
    display: inline-block;
    width: 20px;
    height: 20px;
    background-color: blue;
  }

  /* visualize size contribution */
  .container > div {
    float: left;
    border: solid orange 20px;
    border-style: none solid;
  }
  .container > div > div {
    border-right: solid 20px aqua;
    writing-mode: vertical-rl;
    width: 80px;
    height: 20px;
  }

  /* test width */
  /* content = 100% = 100px = 80px + border */
  /* choose sizes that are different than content to see if how they take effect */
  .raw-percent {
    max-width: 50%;
  }
  .calc-percent {
    max-width: calc(40px + 0%);
  }
  .no-percent {
    max-width: 40px;
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
