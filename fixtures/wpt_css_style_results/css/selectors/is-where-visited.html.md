# css/selectors/is-where-visited.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-where-visited.html"
}
```

## style[0]

```css

  :visited, :link {
    color: black;
  }

  #parent1 :is(:visited) {
    color: green;
  }
  #parent1 :is(:link):nth-child(even) {
    color: magenta;
  }

  :is(#parent2 .a + :visited, #parent2 .e:link) {
    color: tomato;
  }

  main :is(a:visited > :where(.a + span + span, .f)) {
    color: skyblue;
  }

  main :is(a:link > :where(.a + span + span, .f)) {
    color: blue;
  }

  main :is(a:is(:link, :visited) > :where(.b, .e)) {
    color: maroon;
  }
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
