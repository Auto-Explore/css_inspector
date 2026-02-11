# css/css-cascade/scope-pseudo-element-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-pseudo-element-ref.html"
}
```

## style[0]

```css

  body > div {
    display: inline-block;
    width: 100px;
    height: 100px;
    border: 1px solid black;
    vertical-align:top;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  #before_test > main {
    background-color: skyblue;
  }
  #before_test > main::before {
    content: "B";
    width: 20px;
    height: 20px;
    display: inline-block;
    background-color: tomato;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  #after_test > main {
    background-color: skyblue;
  }
  #after_test > main::after {
    content: "A";
    width: 20px;
    height: 20px;
    display: inline-block;
    background-color: tomato;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

  #marker_test li {
    background-color: skyblue;
  }
  #marker_test li::marker {
    content: "M";
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
