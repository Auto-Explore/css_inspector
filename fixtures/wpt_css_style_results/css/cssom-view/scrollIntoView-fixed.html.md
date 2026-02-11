# css/cssom-view/scrollIntoView-fixed.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/scrollIntoView-fixed.html"
}
```

## style[0]

```css

      body {
        width: 1000vw;
        height: 1000vh;
        /* stripes so we can see scroll offset more easily */
        background: repeating-linear-gradient(
          45deg,
          #A2CFD9,
          #A2CFD9 100px,
          #C3F3FF 100px,
          #C3F3FF 200px
        );
      }

      .fixedContainer {
        position: fixed;
        bottom: 10px;
        left: 10px;
        width:  150px;
        height: 150px;
        background-color: coral;
      }

      .fixedContainer.scrollable {
        overflow: auto;
        left: unset;
        right: 10px;
      }

      button {
        position: absolute;
        margin: 5px;
      }

      .target {
        position: absolute;
        width: 10px;
        height: 10px;
        background-color: blue;
        left: 50%;
        top: 50%;
      }

      .scrollable .target {
        left: 200%;
        top: 200%;
      }

      iframe {
        width: 96vw;
        height: 300px;
        position: absolute;
        left: 2vw;
        top: 100px;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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

## style[1]

```css

          body {
            height: 200vh;
            width: 200vw;
            /* stripes so we can see scroll offset more easily */
            background: repeating-linear-gradient(
              45deg,
              #C3A2D9,
              #C3A2D9 50px,
              #E5C0FF 50px,
              #E5C0FF 100px
            );
          }
          .fixedContainer {
            position: fixed;
            bottom: 10px;
            left: 10px;
            width:  150px;
            height: 150px;
            background-color: coral;
          }

          .fixedContainer.scrollable {
            overflow: auto;
            left: unset;
            right: 10px;
          }

          button {
            position: absolute;
            margin: 5px;
          }

          .target {
            position: absolute;
            width: 10px;
            height: 10px;
            background-color: blue;
            left: 50%;
            top: 50%;
          }

          .scrollable .target {
            left: 200%;
            top: 200%;
          }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
