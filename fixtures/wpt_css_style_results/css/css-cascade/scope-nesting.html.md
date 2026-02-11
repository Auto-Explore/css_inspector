# css/css-cascade/scope-nesting.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-nesting.html"
}
```

## style[0]

```css

      /* (& > .b) behaves like (:where(:scope) > .b), due & mapping to :where(:scope).*/
      @scope (.a) to (& > .b) {
        :scope { z-index:1; }
      }

      /* Should not match, since <scope-end> refers to the scope itself. */
      @scope (.a) to (.b&) {
        :scope { z-index:42; }
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

      /* (.b) behaves like (:scope .b), due :scope being prepended
          implicitly. */
      @scope (.a) to (.b) {
        :scope { z-index:1; }
      }

      /* Should not match, since <scope-end> refers to the scope itself. */
      @scope (.a) to (.b:scope) {
        :scope { z-index:42; }
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

      @scope (.a) to (> .b) {
        *, :scope { z-index:1; }
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

      @scope (#div) {
        & {
          z-index: 1;
          & {
            z-index: 2;
          }
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

## style[4]

```css
 select inclusive descendants of the scope root');
</script>

<template id=test_parent_in_pseudo_scope>
  <div>
    <style>
      @scope (#div) {
        :scope {
          z-index: 1;
          & {
            z-index: 2;
          }
        }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

      @scope (#div) {
        :scope {
          z-index: 1;
          & {
            & {
              z-index: 2;
            }
          }
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

## style[6]

```css

      .a {
        @scope (.b) {
          .c { z-index: 1; }
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

      .a {
        @scope (&.b) {
          :scope { z-index: 1; }
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

## style[8]

```css

      .a {
        @scope (.b) {
           /* The & points to <scope-start>, which contains an implicit &
              which points to .a. */
           &.c { z-index: 1; }
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

## style[9]

```css

      .a {
        @scope (.b) {
          z-index: 1;
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

## style[10]

```css

      .a {
        /* The '& .b' selector is wrapped in :where() to prevent a false
           positive when the implementation incorrectly wraps
           the z-index declaration in a rule with &-behavior
           rather than :where(:scope)-behavior. */
        @scope (:where(& .b)) {
          z-index: 1; /* Should win due to proximity */
        }
      }
      :where(.b) { z-index: 2; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[11]

```css

      @scope (.a) {
        .b {
          /* When nesting, because we’re  inside a defined scope,
             the `:scope` should reference the scoping root node properly, and
             check for the presence of an extra class on it, essentially
             being equal to `:scope.x .b { z-index: 1 }`. */
          &:is(:scope.x *) {
            z-index: 1;
          }
          /* This should not match, as we have a defined scope, and should
             not skip to the root. */
          &:is(:root:scope *) {
            z-index: 2;
          }
        }
        /* The nested case can be though of the following when expanded: */
        .c:is(:scope.x *) {
          z-index: 3;
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

## style[12]

```css

      @scope (.b) {
        .a:not(:scope) {
          & :scope {
            z-index: 1;
          }
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

## style[13]

```css

      @scope (.b) {
        .a:not(:scope) {
          :scope { /* & implied */
            z-index: 1;
          }
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

## style[14]

```css

      @scope (.b) {
        .a:not(:scope) {
          > :scope { /* & implied */
            z-index: 1;
          }
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

## style[15]

```css

      @scope (.a) {
        .b:not(:scope) {
          @media (width) {
            z-index: 1;
          }
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

## style[16]

```css

      @scope (.a) {
        @scope(#descendant) {
          :scope {
            z-index: 1;
          }
        }
        @scope (> #child) {
          :scope {
            z-index: 1;
          }
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

## style[17]

```css

      .nest {
        @scope {
          #child {
            color: green;
          }
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

## style[18]

```css

  .a {
    @scope (.b) {
      #child {
        color: red;
      }
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

## style[19]

```css

  .a {
    @scope (.b) {
      #child {
        color: red;
      }
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

## style[20]

```css

  :where(.a) {
    color: red;
  }
  @scope (.a) {
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[21]

```css

  #child {
    color: green; /* Specificity: (1, 0, 0) */
  }
  .b {
    #child {
      @scope (&) {
        --x: 1;
        color: red; /* Specificity: (0, 0, 0), effectively :where(:scope) */
      }
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

## style[22]

```css

    :where(.x) {
      background-color: black;
    }
    .a {
      @scope (&) {
        :scope .x { background-color: green; }
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

## style[23]

```css

    :where(.x) {
      background-color: black;
    }
    .a {
      @scope (&) {
        :scope .x {
          .unused {}
          /* CSSNestedDeclarations { */
          background-color: green;
          /* } { */
        }
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
