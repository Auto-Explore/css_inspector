# css/CSS2/positioning/relpos-calcs-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/relpos-calcs-001.xht"
}
```

## style[0]

```css

        div {
            height: 120px;
            width: 120px;
        }
        .container {
            margin-top: -60px;
        }
        .outer {
            background: red;
            position: relative;
            bottom: -50%;
            /*
            div.outer's used bottom value is -60px
            div.outer's used top value is 60px
            div.outer's computed bottom value is -50%
            but
            div.outer's computed top value is auto and not 50%!
            */
        }
        .inner {
            background: green;
            position: relative;
            top: inherit;
            /* using inheritance to test computed vs. used */
            /* What is inherited is the computed top value of
            its containing block, which is auto!
            */
        }
        .control {
          background: red;
          margin-top: -60px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
