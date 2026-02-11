# css/CSS2/positioning/relpos-calcs-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/relpos-calcs-007.xht"
}
```

## style[0]

```css

        div {
            height: 80px;
            width: 120px;
        }
        .outer {
            background: green;
            position: relative;
            top: 50%;
            bottom: 50%;

            /*
            div.outer's used top value is 40px
            div.outer's used bottom value is -40px
            div.outer's computed top value is 50%
            div.outer's computed bottom value is 50%
            */

        }
        .inner {
            background: green;
            position: relative;
            bottom: inherit; /* using inheritance to test computed vs. used */
            /* div.inner's bottom inherits div.outer's computed bottom value */
        }
        .control {
          background: red;
          margin-top: -80px;
          height: 120px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
