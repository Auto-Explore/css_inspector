# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-text-fragment-navigation-target.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-with-text-fragment-navigation-target.html"
}
```

## style[0]

```css

    :root {
      margin: 0px;
    }

    #spacer {
      height: 100vh;
      width: 100px;
    }

    #top_box {
      width: 100px;
      height: 60vh;
      background-color: blue;
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

    #fragment_target {
      width: 100px;
      height: 100px;
      background-color: red;
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
