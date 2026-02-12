# css/css-text-decor/reference/text-underline-position-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/reference/text-underline-position-001-ref.html"
}
```

## style[0]

```css

        .outer {
            margin: 1em;
            clear: left;
        }
        .outer > div {
            margin: .25em;
            padding: .25em;
            float: left;
            border: 1px dotted gray;
        }
        u {
            text-decoration: 2px green underline;
            text-underline-offset: .25em;
        }
        .test1 > div {
            writing-mode: initial;
        }
        .test2 > div {
            writing-mode: sideways-rl;
        }
        .test3 > div {
            writing-mode: sideways-lr;
        }
        .test4 > div {
            writing-mode: vertical-rl;
            text-orientation: sideways;
        }
        .test5 > div {
            writing-mode: vertical-lr;
            text-orientation: sideways;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
