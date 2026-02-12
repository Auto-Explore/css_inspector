# css/css-position/sticky/position-sticky-fixed-ancestor-iframe-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-fixed-ancestor-iframe-ref.html"
}
```

## style[0]

```css

  body,html {
    margin: 0;
    width: 100%;
    height: 100%;
  }

  iframe {
    margin: 10px;
    width: 90%;
    height: 90%;
    border: 1px solid black;
  }

  .spacer {
    height: 120vh;
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

    body,html {
      margin: 0;
      width: 100%;
      height: 100%;
    }

    .sticky {
      background: green;
      width: 100px;
      height: 10%;
    }

    .spacer {
      height: calc(25vh - 10%);
    }
    .long {
      height: 600vh;
    }

    .position-parent {
      position: absolute;
      display: flex;
      align-items: center;
      justify-content: center;
      width: 100%;
      height: 100%;
      top: 100vh;
      background-color: lightgrey;
    }

    .container {
      width: 100px;
      height: 100%;
      background-color: grey;
    }

    button {
      position: fixed;
      left: 20px;
      top: 20px;
    }

    .fixed {
      position: fixed;
      top: 25vh;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
