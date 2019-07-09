declare module "figgy-pudding" {
  function figgyPudding<S extends figgyPudding.Specs = {}, O extends figgyPudding.Options = {}>(
    specs?: S,
    opts?: O,
  ): figgyPudding.PuddingFactory<S, O>;

  namespace figgyPudding {
    interface Options {
      other?(key: string): boolean;
    }

    type OtherOpt = Required<Pick<Options, "other">>;

    type Specs = {
      [K in string]: string | Spec;
    };
    interface Spec {
      default?: any;
    }

    type SpecWithDefault = Required<Pick<Spec, "default">>;
    type WidenPrimitive<T> = T extends string ? string : T extends number ? number : T extends boolean ? boolean : T;
    type SpecDefault<S> = S extends { default(): infer R }
      ? WidenPrimitive<R>
      : S extends { default: infer D }
      ? D
      : unknown;

    interface MapLike<K, V> {
      get(key: K): V | undefined;
    }

    type AvailableKeys<S, O> = O extends OtherOpt ? string : keyof S;

    type Proxy<S, O> = {
      [K in keyof S]: SpecDefault<S[K]>;
    } &
      (O extends { other(key: string): boolean }
        ? {
            [key: string]: unknown;
          }
        : {});

    export type ProxyFiggyPudding<S, O> = Readonly<Proxy<S, O>> & FiggyPudding<S, O>;

    type PuddingFactory<S, O> = (...providers: any[]) => ProxyFiggyPudding<S, O>;

    interface FiggyPuddingConstructor {
      new <S extends Specs, O extends Options>(specs: S, opts: O, providers: any[]): FiggyPudding<S, O>;
    }

    interface FiggyPudding<S, O> {
      readonly __isFiggyPudding: true;
      readonly [Symbol.toStringTag]: "FiggyPudding";

      get<K extends AvailableKeys<S, O>>(key: K): K extends keyof S ? SpecDefault<S[K]> : unknown;
      concat<P extends any[]>(...providers: P): ProxyFiggyPudding<S, O>;
      toJSON(): {
        [K in AvailableKeys<S, O>]: K extends keyof S ? SpecDefault<S[K]> : unknown;
      };
      forEach<This = this>(
        fn: (this: This, value: unknown, key: AvailableKeys<S, O>, opts: this) => void,
        thisArg?: This,
      ): void;
      entries(matcher: (key: string) => boolean): IterableIterator<[string, unknown]>;
      entries(): IterableIterator<[AvailableKeys<S, O>, unknown]>;
      [Symbol.iterator](): IterableIterator<[AvailableKeys<S, O>, unknown]>;
      keys(): IterableIterator<AvailableKeys<S, O>>;
      values(): IterableIterator<unknown>;
    }
  }

  export default figgyPudding;
  export type ProxyFiggyPudding<S, O> = figgyPudding.ProxyFiggyPudding<S, O>;
}
