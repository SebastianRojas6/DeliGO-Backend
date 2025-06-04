/**
 * A trait that defines a mapping from a Data Transfer Object (DTO)
 * to a Presenter. This is typically used to convert internal objects
 * into forms suited for external presentation or response.
 *
 * # Type Parameters
 * - `DTO`: The input type representing the data transfer object.
 * - `Presenter`: The output type suited for presentation or response.
 *
 * # Required Method
 * - `fn to_presenter(object: DTO) -> Presenter`:
 *   Converts a given object of type `DTO` into a `Presenter` type.
 */
pub trait ApiOutMapper<DTO, Presenter> {
    fn to_presenter(object: DTO) -> Presenter;
}

/**
 * A trait that defines a mapping from an input payload to a Data Transfer Object (DTO).
 * This is typically used to convert incoming data into a format suitable for processing
 * within the application.
 *
 * # Type Parameters
 * - `Payload`: The input type representing the data payload.
 * - `DTO`: The output type representing the data transfer object.
 *
 * # Required Method
 * - `fn to_api(payload: Payload) -> DTO`:
 *   Converts a given payload of type `Payload` into a `DTO` type.
 */
pub trait ApiInMapper<Payload, DTO> {
    fn to_api(payload: Payload) -> DTO;
}