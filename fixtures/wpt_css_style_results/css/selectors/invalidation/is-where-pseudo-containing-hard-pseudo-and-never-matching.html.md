# css/selectors/invalidation/is-where-pseudo-containing-hard-pseudo-and-never-matching.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/is-where-pseudo-containing-hard-pseudo-and-never-matching.html"
}
```

## style[0]

```css

.container {
  color: grey;
}

#subject1:is(.container:has(.descendant):focus-within, .never-matches) {
  color: red;
}

#subject2:where(.container:has(.descendant):focus-within, .never-matches) {
  color: orangered;
}

#subject3:is(:nth-child(1 of .container):focus-within, .never-matches) {
  color: darkred;
}

#subject4:where(:nth-child(1 of .container):focus-within, .never-matches) {
  color: pink;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
