# css/css-scroll-anchoring/heuristic-with-offset-update.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/heuristic-with-offset-update.html"
}
```

## style[0]

```css

        #scroller {
            overflow: scroll;
            height: 500px;
            height: 500px;
        }
        #before {
            height: 200px;
        }
        #anchor {
            position: relative;
            width: 200px;
            height: 200px;
            margin-bottom: 500px;
            background-color: blue;
            /*
             * To trigger the Gecko bug that's being regression-tested here, we
             * need 'top' to start out at a non-'auto' value, so that the
             * dynamic change can trigger Gecko's "RecomputePosition" fast path
             */
            top: 0px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
