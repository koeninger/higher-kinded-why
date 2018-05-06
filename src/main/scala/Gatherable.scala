import scala.language.higherKinds

trait Gatherable[F[_]] {
  def gather[G[_], A](x: F[G[A]]): G[F[A]]
}
