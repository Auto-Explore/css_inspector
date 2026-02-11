# css/selectors/invalidation/is-where-pseudo-containing-hard-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/is-where-pseudo-containing-hard-pseudo.html"
}
```

## style[0]

```css

.container {
  color: grey;
}

#subject1:is(.other-match, :has(.descendant)) {
  color: red;
}

#subject1:is(.parent > .other-match, .parent > :has(.descendant)) {
  color: orangered;
}

#subject2:where(.other-match, :has(.descendant)) {
  color: darkred;
}

#subject2:where(.parent > .other-match, .parent > :has(.descendant)) {
  color: pink;
}

#subject3:is(.other-match, :nth-child(1000 of .another-match)) {
  color: green;
}

#subject3:is(.parent > .other-match, .parent > :nth-child(1000 of .another-match)) {
  color: lightgreen;
}

#subject4:where(.other-match, :nth-child(1000 of .another-match)) {
  color: darkgreen;
}

#subject4:where(.parent > .other-match, .parent > :nth-child(1000 of .another-match)) {
  color: yellowgreen;
}
```

```json
{
  "errors": 7,
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
