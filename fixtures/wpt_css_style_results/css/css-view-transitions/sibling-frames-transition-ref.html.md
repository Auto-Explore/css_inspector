# css/css-view-transitions/sibling-frames-transition-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/sibling-frames-transition-ref.html"
}
```

## style[0]

```css

body {
  background: green;
}

#first {
  position: fixed;
  top: 0;
  left: 0;
  width: 50vw;
  height: 50vh;
}

#second {
  position: fixed;
  top: 50vh;
  left: 0;
  width: 50vw;
  height: 50vh;
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

  body {
    background: lightblue;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  body {
    background: magenta;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
