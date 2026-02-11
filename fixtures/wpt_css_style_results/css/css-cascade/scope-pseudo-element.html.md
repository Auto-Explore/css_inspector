# css/css-cascade/scope-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-pseudo-element.html"
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

  @scope (#before_test > main) {
    :scope {
      background-color: skyblue;
    }
    :scope::before {
      content: "B";
      width: 20px;
      height: 20px;
      display: inline-block;
      background-color: tomato;
    }
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

  @scope (#after_test > main) {
    :scope {
      background-color: skyblue;
    }
    :scope::after {
      content: "A";
      width: 20px;
      height: 20px;
      display: inline-block;
      background-color: tomato;
    }
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

  @scope (#marker_test li) {
    :scope {
      background-color: skyblue;
    }
    :scope::marker {
      content: "M";
    }
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
