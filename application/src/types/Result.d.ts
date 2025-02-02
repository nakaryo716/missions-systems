export type Ok<T> = { ok: true, value: T};
export type Err<E> = { ok: false, err: E};

export type Result<T, E> = Ok<T> | Err<E>;
