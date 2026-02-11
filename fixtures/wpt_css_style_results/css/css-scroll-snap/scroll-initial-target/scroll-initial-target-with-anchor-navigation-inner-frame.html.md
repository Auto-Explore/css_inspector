# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-anchor-navigation-inner-frame.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-anchor-navigation-inner-frame.html"
}
```

## style[0]

```css

    :root, body {
      margin: 0px;
    }

    #spacer {
      height: 100vh;
      width: 100px;
    }

    #top_box {
      width: 100px;
      height: 60vh;
      background-color: red;
    }
    #middle_box {
      width: 100px;
      height: 60vh;
      scroll-initial-target: nearest;
      background-color: purple;
    }
    #bottom_box {
      width: 100px;
      height: 60vh;
      background-color: yellow;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “scroll-initial-target”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
