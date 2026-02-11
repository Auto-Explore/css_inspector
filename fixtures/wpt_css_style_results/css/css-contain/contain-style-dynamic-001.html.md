# css/css-contain/contain-style-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-dynamic-001.html"
}
```

## style[0]

```css

  /* Selectors for contain */
  #none .wrapper {
      contain: none;
  }
  #style .wrapper {
      contain: style;
  }
  #none_to_style .wrapper {
      contain: none;
  }
  #style_to_none .wrapper {
      contain: style;
  }

  /* Selectors for testing counters */
  .set_counter_to_9 {
      counter-set: testcounter 9;
  }
  .increment_counter {
      counter-increment: testcounter;
  }
  .set_counter_to_10 {
      counter-set: testcounter 10;
  }
  span.print_counter::after {
      font: 25px/1 Ahem;
      content: counters(testcounter, ".");
  }

  /* Selectors for testing quotes */
  .open_quote::after {
      content: open-quote;
  }
  .close_quote::after {
      content: close-quote;
  }
  .no_open_quote::after {
      content: no-open-quote;
  }
  .no_close_quote::after {
      content: no-close-quote;
  }
  span.print_quotes::before, span.print_quotes::after {
      font: 25px/1 Ahem;
      quotes: "A" "" "BB" "" "CCC" "" "DDDD" "" "EEEEE" "" "FFFFF" "" "GGGGGG" "" "HHHHHHH" "" "IIIIIIII" "" "JJJJJJJJJ" "";
  }
  span.print_quotes::before {
      content: open-quote;
  }
  span.print_quotes::after {
      content: no-close-quote;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
