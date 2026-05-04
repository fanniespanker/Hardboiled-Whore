# CNML Core Specification v0.1.3

## Foundations

CNML is a constrained XML vocabulary with semantic rules.

## Discourse

-   Default mode: implicit `<narrative>`{=html}
-   `<irreal>`{=html}: non-actualized discourse, non-nesting, may span
    units

## Discourse Units

-   Derived from blank-line separation
-   Computed per container

## Expression Modality

-   `<say>`{=html}: spoken
-   `<think mode="instinct">`{=html}: internal cognition
-   `<gesture>`{=html}: nonverbal

## Prosody

-   `<soft>`{=html}: attenuation
-   `<strong>`{=html}: intensity
-   `<emph>`{=html}: emphasis

## Integrity

-   `<redacted>`{=html}, `<censored>`{=html}, `<glitched>`{=html}

## Rules

-   Inline elements compose
-   No cross-boundary inheritance
