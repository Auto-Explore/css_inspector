# css/CSS2/positioning/relpos-calcs-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/relpos-calcs-003.xht"
}
```

## style[0]

```css

        div {
            height: 120px;
            width: 120px;
        }
        .container {
            margin-left: -60px;
        }
        .outer {
            background: red;
            position: relative;
            right: -50%;
            /*
            div.outer's used right value is -60px
            div.outer's used left value is 60px
            div.outer's computed right value is -50%
            but
            div.outer's computed left value is auto and not 50%!
            */

        }
        .inner {
            background: green;
            position: relative;
            left: inherit; /* using inheritance to test computed vs. used */
        }
        .control {
          background: red;
          margin-bottom: -120px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
