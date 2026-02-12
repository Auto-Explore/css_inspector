# css/css-scroll-snap/scroll-initial-target/scroll-initial-target-shadow-dom.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-initial-target/scroll-initial-target-shadow-dom.tentative.html"
}
```

## style[0]

```css

      .space {
        width: 50px;
        height: 500px;
      }
      .scroller {
        width: 100px;
        height: 100px;
        overflow: scroll;
        border: solid 2px;
      }
      .purpleborder {
        border: solid 2px purple;
      }
      .target {
        scroll-initial-target: nearest;
        width: 50px;
        height: 50px;
        background-color: green
      }
      #wrapper {
        /* Hide everything initially to ensure that the test sees the scroll */
        /* events from the scrolls to the scroll-initial-targets.            */
        display: none;
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

            .space {
              width: 50px;
              height: 500px;
            }
            .scroller {
              width: 100px;
              height: 100px;
              overflow: scroll;
              border: solid 2px red;
            }
            .target {
              scroll-initial-target: nearest;
              width: 50px;
              height: 50px;
              background-color: green
            }
          
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
