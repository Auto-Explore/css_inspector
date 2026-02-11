# css/css-nesting/nested-declarations-matching.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nested-declarations-matching.html"
}
```

## style[0]

```css

  .trailing {
    --x: FAIL;
    & { --x: FAIL; }
    --x: PASS;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  .trailing_no_leading {
    & { --x: FAIL; }
    --x: PASS;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

  .trailing_multiple {
    --x: FAIL;
    --y: FAIL;
    --z: FAIL;
    --w: FAIL;
    & { --x: FAIL; }
    --x: PASS;
    --y: PASS;
    & { --z: FAIL; }
    --z: PASS;
    --w: PASS;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

  .trailing_specificity {
    --x: FAIL;
    :is(&, div.nomatch2) { --x: PASS; } /* Specificity: (0, 1, 1) */
    --x: FAIL; /* Specificity: (0, 1, 0) */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

  #nomatch, .specificity_top_level {
    --x: FAIL;
    :is(&, div.nomatch2) { --x: PASS; } /* Specificity: (0, 1, 1) */
    --x: FAIL; /* Specificity: (0, 1, 0). In particular, this does not have
               specificity like :is(#nomatch, .specificity_top_level). */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

  #nomatch, .specificity_top_level_max, div.specificity_top_level_max {
    --x: FAIL;
    :is(:where(&), div.nomatch2) { --x: FAIL; } /* Specificity: (0, 1, 1) */
    --x: PASS; /* Specificity: (0, 1, 1) (for div.specificity_top_level_max) */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

  .nested_pseudo::after {
    --x: FAIL;
    @media (width > 0px) {
      --x: PASS;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css

  #nomatch, .nested_group_rule {
    --x: FAIL;
    @media (width > 0px) {
      --x: FAIL; /* Specificity: (0, 1, 0) */
    }
    --x: PASS;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css

  .nested_scope_rule {
    div:where(&) { /* Specificity: (0, 0, 1) */
      --x: PASS;
    }
    @scope (&) {
      --x: FAIL; /* Specificity: (0, 0, 0) */
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[9]

```css

  .nested_scope_rule_trailing {
    div:where(&) { /* Specificity: (0, 0, 1) */
      --x: PASS;
    }
    @scope (&) {
      --ignored: 1;
      .ignored {}
      --x: FAIL; /* Specificity: (0, 0, 0) */
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[10]

```css

  .set_parent_selector_text {
    div {
      color: red;
    }
    .a1 {
      .ignored {}
      color: green;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
