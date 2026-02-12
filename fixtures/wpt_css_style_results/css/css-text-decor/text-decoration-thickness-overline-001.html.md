# css/css-text-decor/text-decoration-thickness-overline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-thickness-overline-001.html"
}
```

## style[0]

```css

        #box{
            font: 20px/1 Ahem;
            overflow: hidden;
            height: 1em;
            width: 4em;
            background-color: red;
        }
        /*
         * This is testing to ensure that the overline
         * "grows up" and covers the red in the box
         */
        #text{
            color: transparent;
            position: relative;
            top: 3em;
            text-decoration: green overline;
            text-decoration-skip-ink: none;
            text-decoration-thickness: 4em;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
